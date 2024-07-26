// load config file in json format
use actix_web::{get, post, web, HttpResponse, Responder};
use std::fs::File;
use std::io::prelude::*;
pub mod entity;
use entity::*;

#[get("/config")]
pub async fn index() -> impl Responder {
    let config = check_file().await;
    HttpResponse::Ok().body(config)
}

#[post("/config/update_server")]
pub async fn update_server(post: web::Json<UpdateServer>) -> impl Responder {
    // todo: desenvolver a função de atualização do servidor
    let file = File::open("config.json").unwrap();
    let file = std::io::BufReader::new(file);
    let mut config: Config = serde_json::from_reader(file).unwrap();
    config.server.server_host = post.server_host.clone();
    config.server.server_port = post.server_port;
    let config = serde_json::to_string(&config).unwrap();
    let mut file = File::create("config.json").unwrap();
    file.write_all(config.as_bytes()).unwrap();
    HttpResponse::Ok().body(config)
}

#[post("/config/update_sat")]
pub async fn update_sat(post: web::Json<UpdateSat>) -> impl Responder {
    // todo: desenvolver a função de atualização do sat
    let file = File::open("config.json").unwrap();
    let file = std::io::BufReader::new(file);
    let mut config: Config = serde_json::from_reader(file).unwrap();
    config.sat.cnpj = post.cnpj.clone();
    config.sat.sign_ac = post.sign_ac.clone();
    config.sat.codigo_ativacao = post.codigo_ativacao.clone();
    config.sat.dll_path = post.dll_path.clone();
    config.sat.dll_load_enviroment = post.dll_load_enviroment.clone();
    config.sat.dll_convention = post.dll_convention.clone();
    config.sat.dll_maker = post.dll_maker.clone();
    let config = serde_json::to_string(&config).unwrap();
    let mut file = File::create("config.json").unwrap();
    file.write_all(config.as_bytes()).unwrap();
    HttpResponse::Ok().body(config)
}
// check if the file exists, if not create it and send the default configuration
pub async fn check_file() -> String {
    let file = File::open("config.json");
    if file.is_err() {
        println!("Arquivo de configuração não encontrado, criando arquivo de configuração padrão");
        let mut file = File::create("config.json").unwrap();
        let default_config = entity::Config::new();
        let default_config = serde_json::to_string(&default_config).unwrap();
        file.write_all(default_config.as_bytes()).unwrap();
        return default_config;
    }
    let file = file.unwrap();
    let mut file = std::io::BufReader::new(file);
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

// retorna os dados do arquivo de configuração 1 a 1
pub fn server_host() -> String {
    let file = File::open("config.json").unwrap();
    let file = std::io::BufReader::new(file);
    let config: entity::Config = serde_json::from_reader(file).unwrap();
    config.server.server_host
}

pub fn server_port() -> u16 {
    let file = File::open("config.json").unwrap();
    let file = std::io::BufReader::new(file);
    let config: entity::Config = serde_json::from_reader(file).unwrap();
    config.server.server_port
}

pub fn version() -> String {
    let file = File::open("config.json").unwrap();
    let file = std::io::BufReader::new(file);
    let config: entity::Config = serde_json::from_reader(file).unwrap();
    config.server.version
}

pub fn dll_path() -> String {
    let file = File::open("config.json").unwrap();
    let file = std::io::BufReader::new(file);
    let config: entity::Config = serde_json::from_reader(file).unwrap();
    config.sat.dll_path
}

pub fn codigo_ativacao() -> String {
    let file = File::open("config.json").unwrap();
    let file = std::io::BufReader::new(file);
    let config: entity::Config = serde_json::from_reader(file).unwrap();
    config.sat.codigo_ativacao
}
