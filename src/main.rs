#[macro_use] extern crate diesel;
use actix_web::{ web, App, HttpServer };
use dotenvy::dotenv;
use std::env;
use tera::Tera;

mod utils;
use utils::static_files::get_statics;

mod views;
use views::home_views::index_view;

mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let host = env::var("HOST").expect("HOST must be set");
    let port: u16 = u16::from_str_radix(&env::var("PORT").unwrap().to_string(), 10).unwrap();

    HttpServer::new(|| {

        let tera: Tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .service(get_statics)
            .service(index_view)
    })
    .bind((host, port))?
    .run()
    .await
}
