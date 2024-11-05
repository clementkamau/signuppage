use std::error::Error;
use crate::{models::NewUser, schema};
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2,
};
use serde::Deserialize;
use schema::users;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection, RunQueryDsl};



type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct SignUpData {
    name: String,
    email: String,
    password: String,
}


fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

#[post("/signup")]
pub async fn sign_up(form: web::Json<SignUpData>,pool:web::Data<DbPool>) -> impl Responder {
    let username = &form.name;
    let email = &form.email;

   
    let hashed_password = match hash_password(&form.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Password hashing failed"),
    };

    
  let new_user: NewUser<'_> = NewUser{
    name: username,
    email:email,
    password_hash: &hashed_password
  };

  let mut conn = pool.get().expect("Failed to get a connection from the pool");
    
  
  diesel::insert_into(users::table)
      .values(&new_user)
      .execute(&mut conn)
      .expect("Error saving new user");

  println!("New user: {}, email: {}", username, email);

    HttpResponse::Ok().json(format!("User {} signed up successfully!", username))
}
