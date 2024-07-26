use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub server_host: String,
    pub server_port: u16,
    pub version: String,
}

impl Server {
    pub fn new() -> Self {
        Self {
            server_host: "0.0.0.0".to_string(),
            server_port: 3030,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sat {
    pub cnpj: String,
    pub sign_ac: String,
    pub codigo_ativacao: String,
    pub dll_path: String, // C:\sat.dll (atenção: a dll deve ser 64 bits)
    pub dll_load_enviroment: String, // win64, linux64, macos64, android64. (32 bits not supported)
    pub dll_convention: String, // cdecl, stdcall, fastcall
    pub dll_maker: String, // ControlID, Sweda, Daruma, Elgin, Tanca, Bematech
}

impl Sat {
    pub fn new() -> Self {
        Self {
            cnpj: "11111111111111".to_string(),
            sign_ac: "AC_SIGNATURE".to_string(),
            codigo_ativacao: "00000000".to_string(),
            dll_path: "C:\\sat.dll".to_string(), // 64bits
            dll_load_enviroment: "win64".to_string(),
            dll_convention: "stdcall".to_string(),
            dll_maker: "DEMO".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub sat: Sat,
}

impl Config {
    pub fn new() -> Self {
        Self {
            server: Server::new(),
            sat: Sat::new(),
        }
    }
}

// update server configuration
#[derive(Serialize, Deserialize)]
pub struct UpdateServer {
    pub server_host: String,
    pub server_port: u16,
}

// update sat configuration
#[derive(Serialize, Deserialize)]
pub struct UpdateSat {
    pub cnpj: String,
    pub sign_ac: String,
    pub codigo_ativacao: String,
    pub dll_path: String,
    pub dll_load_enviroment: String,
    pub dll_convention: String,
    pub dll_maker: String,
}
