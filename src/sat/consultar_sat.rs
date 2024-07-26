use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::os::raw::c_char;

use crate::sat::entity::{ConsultarSatData, RespConsultarSat};

pub async fn exec_consultar_sat() -> String {
    let numero_sessao = crate::sat::common::cod_sessao();
    let retorno = unsafe {
        let load_dll = Library::new(crate::config::dll_path().as_str());
        if load_dll.is_err() {
            return format!(
                "
            Erro ao carregar a DLL: {:?}.
            DLL Path: {:?}
            ",
                load_dll.err(),
                crate::config::dll_path()
            );
        }
        let load_dll = load_dll.unwrap();

        let dll_cmd: Symbol<extern "C" fn(i32) -> *const c_char> =
            match load_dll.get(b"ConsultarSAT") {
                Ok(func) => func,
                Err(_) => return "Erro ao carregar a função".to_string(),
            };

        let c_str_ptr = dll_cmd(numero_sessao);
        if c_str_ptr.is_null() {
            return "Erro ao executar a função".to_string();
        }
        let c_str = CStr::from_ptr(c_str_ptr);
        let retorno_str = c_str.to_string_lossy().into_owned();
        retorno_str
    };
    retorno
}

pub async fn json_consultar_sat(income_response: String) -> RespConsultarSat {
    // exemplo de response 577716|08000|SAT em operação||
    let response: Vec<&str> = income_response.split("|").collect();

    if response.len() < 3 {
        RespConsultarSat {
            error: 1,
            code: Some(1),
            msg: format!("{:?}", response),
            data: None,
        }
    } else {
        RespConsultarSat {
            error: 0,
            code: Some(0),
            msg: "Sucesso ao consultar o equipamento SAT".to_string(),
            data: Some(ConsultarSatData {
                numero_sessao: response[0].parse().unwrap(),
                cod_retorno: response[1].parse().unwrap(),
                mensagem: response[2].to_string().clone(),
            }),
        }
    }
}
