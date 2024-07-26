#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]
use actix_cors::Cors;
use actix_web::{App, HttpServer};

mod config;
mod home;
mod migrations;
mod sat;
mod tasks;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tasks::init().await;
    println!(
        "Server at http://{}:{}/",
        crate::config::server_host(),
        crate::config::server_port()
    );

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard(),
            )
            .service(config::index)
            .service(home::index)
            .service(sat::consultar_sat_index)
            .service(sat::consultar_status_operacional_index)
    })
    .bind((crate::config::server_host(), crate::config::server_port()))?
    .run()
    .await
}
