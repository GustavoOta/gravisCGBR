#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]
use actix_cors::Cors;
use actix_web::{App, HttpServer};

const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3030;
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod home;
pub mod migrations;
pub mod tasks;
pub mod tests;

// adicionar modulos que estao no arquivo mod.rs

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server at http://{}:{}/", SERVER_HOST, SERVER_PORT);
    tasks::init().await;
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard(),
            )
            .service(home::index)
    })
    .bind((SERVER_HOST, SERVER_PORT))?
    .run()
    .await
}
