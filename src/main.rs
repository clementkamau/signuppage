mod handlers;
mod db;
mod models;
mod schema;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use handlers::sign_up;

use std::env;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("database url should be set in the .env");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = r2d2::Pool::builder().build(manager).expect("failed to create a pool");


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(sign_up)
    })
    .bind("127.0.0.1:8080")? 
    .run()
    .await
}
