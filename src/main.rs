#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// adicionar arquivo de configurações

mod migrations;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // tests in main function execute and stop the program
    let tests_on_off = false;
    if tests_on_off {
        tests::index().await;
        //stop program execution
        return Ok(());
    }

    workers().await;

    let port = 3030;
    println!("Starting server at http://0.0.0.0:{}/", &port);

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard(),
            )
            .service(home)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[get["/"]]
async fn home() -> impl Responder {
    HttpResponse::Ok().body(
        "Gravis CGBR - Sistemas de Gestao Comercial ERP - CNPJ: 10.868.122/0001-58".to_string()
            + " - Versao: "
            + VERSION,
    )
}

async fn workers() {
    // ********************* SYSTEM UPDATE ********************* //
    //system::exec().await;

    // ******** VERIFICAÇÃO DE ESTRUTURA DE DIRETORIOS ********* //
    // common::dirs("C:/gravis/downloads").await;

    // ***************** DATABASE MIGRATIONS ******************* //
    migrations::exec().await;

    // ********************* LOOPS ********************* //

    let _cmd_send = tokio::spawn(async move {
        loop {
            println!("loop init: 0");
            let timeout = 1000;
            tokio::time::sleep(std::time::Duration::from_secs(timeout)).await;
            println!("loop end: {:?}", timeout);
        }
    });
}
/*
exemplo para implementação futura de certificado ssl
use rcgen::{generate_simple_self_signed, CertificateParams};
use std::fs::File;
use std::io::Write;

fn main() {
    // Define the certificate parameters
    let mut params = CertificateParams::new(vec!["192.168.1.1".into()]); // Use your server's IP address or hostname here
    params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;

    // Generate the certificate
    let cert = generate_simple_self_signed(params).unwrap();

    // Write the certificate and key to .pem files
    let mut file = File::create("cert.pem").unwrap();
    file.write_all(cert.serialize_pem().unwrap().as_bytes()).unwrap();

    let mut file = File::create("key.pem").unwrap();
    file.write_all(cert.serialize_private_key_pem().as_bytes()).unwrap();
}

use actix_web::{web, App, HttpServer, Responder};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};
use std::fs::File;
use std::io::BufReader;

async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load public certificate.
    let cert_file = File::open("cert.pem").unwrap();
    let mut cert_reader = BufReader::new(cert_file);
    let cert_chain = certs(&mut cert_reader).unwrap();

    // Load private key.
    let key_file = File::open("key.pem").unwrap();
    let mut key_reader = BufReader::new(key_file);
    let mut keys = pkcs8_private_keys(&mut key_reader).unwrap();

    // Create server config.
    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_rustls("0.0.0.0:8080", config)? // Listen on all interfaces
        .run()
        .await
}

*/
