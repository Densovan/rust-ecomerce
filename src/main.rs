use actix_web::{ web,middleware, App, HttpServer,};
use std::env;
use crate::db::db_connection::db_pool;
use routes::user::register;

mod db;
mod models;
mod routes;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let database = db_pool().await.unwrap();
    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
   //Start Actix server
   HttpServer::new(move || {
    App::new()
    .service(register)
    .app_data(web::Data::new(database.clone()))
    .wrap(middleware::Logger::default())
   })
   .bind("127.0.0.1:8080")?
    .run()
    .await
}