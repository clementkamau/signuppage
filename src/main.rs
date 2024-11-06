use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use actix_files as fs;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;
use crate::handlers::{signup, login}; 

mod handlers;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL should be set in the .env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) 
            .service(fs::Files::new("/static", "static").show_files_listing())
            
            .route("/signup", web::get().to(sign_up_page))
            .route("/login", web::get().to(login_page))
         
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn sign_up_page(req: HttpRequest) -> HttpResponse {
    match fs::NamedFile::open("static/sign_up.html") {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::NotFound().body("Sign up page not found"),
    }
}

async fn login_page(req: HttpRequest) -> HttpResponse {
    match fs::NamedFile::open("static/login.html") {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::NotFound().body("Login page not found"),
    }
}
