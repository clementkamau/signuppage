use crate::{models::{NewUser, User}, schema};
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use diesel::{
    prelude::Queryable, 
    r2d2::{self, ConnectionManager}, 
    ExpressionMethods, 
    PgConnection, 
    RunQueryDsl,
    query_dsl::methods::FilterDsl, // Explicitly importing FilterDsl
};
use log::error;
use schema::users;
use serde::{Deserialize, Serialize};
use std::error::{self, Error};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct SignUpData {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    email: String,
    password: String,
}

fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)? // Hash password with Argon2
        .to_string();
    Ok(password_hash)
}

pub async fn signup(form: web::Json<SignUpData>, pool: web::Data<DbPool>) -> impl Responder {
    let username = &form.name;
    let email = &form.email;

    let hashed_password = match hash_password(&form.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Password hashing failed"),
    };

    let new_user = NewUser {
        name: username,
        email: email,
        password_hash: &hashed_password,
    };

    let mut conn = pool.get().expect("Failed to get a connection from the pool");

    match diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(&mut conn) 
    {
        Ok(_) => HttpResponse::Ok().json(format!("User {} signed up successfully!", username)),
        Err(_) => HttpResponse::InternalServerError().body("Error saving new user"),
    }
}
pub async fn login(form: web::Json<LoginData>, pool: web::Data<DbPool>) -> impl Responder {
    let email = &form.email;
    let password = &form.password;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error"),
    };

    let user: User = match schema::users::table
        .filter(schema::users::email.eq(email))
        .first(&mut conn)
    {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid email or password"),
    };

    let parsed_hash = match argon2::PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch the parsed password"),
    };

    if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
        HttpResponse::Ok().json(format!("User {} has logged in successfully", user.name))
    } else {
        HttpResponse::Unauthorized().body("Invalid email or password")
    }
}
