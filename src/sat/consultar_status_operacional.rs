use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use tokio::task;

use crate::sat::entity::{ConsultarStatusOperacionalData, RespConsultarStatusOperacional};

pub async fn exec_consultar_status_operacional() -> String {
    let numero_sessao = crate::sat::common::cod_sessao();
    let codigo_ativacao = crate::config::codigo_ativacao();
    let dll_path = crate::config::dll_path();

    let retorno = task::spawn_blocking(move || unsafe {
        // Carrega a DLL e considera que o equipamento SAT não está em uso por outra aplicação.
        let load_dll = Library::new(dll_path.as_str());
        if load_dll.is_err() {
            return Err(format!(
                "Erro ao carregar a DLL: {:?}.\nDLL Path: {:?}",
                load_dll.err(),
                dll_path
            ));
        }
        let load_dll = load_dll.unwrap();

        // Aqui você precisa substituir 'TipoFuncao' pelo tipo correto da função que você está tentando chamar.
        let dll_cmd: Symbol<unsafe extern "C" fn(i32, *const c_char) -> *const c_char> =
            match load_dll.get(b"ConsultarStatusOperacional") {
                Ok(func) => func,
                Err(_) => {
                    return Err("Erro ao carregar a função: ConsultarStatusOperacional".to_string());
                }
            };

        let c_str = CString::new(codigo_ativacao).unwrap();
        let c_str_ptr = dll_cmd(numero_sessao, c_str.as_ptr());
        if c_str_ptr.is_null() {
            return Err("Erro ao executar a função, retornou NULL.".to_string());
        }

        let c_str = CStr::from_ptr(c_str_ptr);
        let retorno_str = c_str.to_string_lossy().into_owned();
        Ok(retorno_str)
    })
    .await;
    // transforme retorno em string
    match retorno {
        Ok(r) => {
            if r.is_err() {
                r.err().unwrap()
            } else {
                r.unwrap()
            }
        }
        Err(e) => e.to_string(),
    }
}

pub async fn json_consultar_status_operacional(
    income_response: String,
) -> RespConsultarStatusOperacional {
    let response: Vec<&str> = income_response.split("|").collect();

    if response.len() < 3 {
        if response == [""] {
            return RespConsultarStatusOperacional {
                error: 1,
                code: Some(1),
                msg: "Erro ao consultar o equipamento SAT. Possível erro de comunicação com o equipamento SAT. Causas comuns: Equipamento desligado, cabo desconectado, equipamento em uso por outra aplicação.".to_string(),
                data: None,
            };
        }
        RespConsultarStatusOperacional {
            error: 1,
            code: Some(1),
            msg: format!("{:?}", response),
            data: None,
        }
    } else {
        RespConsultarStatusOperacional {
            error: 0,
            code: Some(0),
            msg: "Sucesso ao consultar o equipamento SAT".to_string(),
            data: Some(ConsultarStatusOperacionalData {
                numero_sessao: response[0].parse().unwrap(),
                cod_retorno: response[1].parse().unwrap(),
                mensagem: response[2].to_string().clone(),
                cod: response[3].to_string().clone(),
                mensagem_sefaz: response[4].to_string().clone(),
                n_serie: response[5].to_string().clone(),
                tipo_lan: response[6].to_string().clone(),
                lan_ip: response[7].to_string().clone(),
                lan_mac: response[8].to_string().clone(),
                lan_mask: response[9].to_string().clone(),
                lan_gw: response[10].to_string().clone(),
                lan_dns_1: response[11].to_string().clone(),
                lan_dns_2: response[12].to_string().clone(),
                status_lan: response[13].to_string().clone(),
                nivel_bateria: response[14].to_string().clone(),
                mt_total: response[15].to_string().clone(),
                mt_usada: response[16].to_string().clone(),
                dh_atual: response[17].to_string().clone(),
                ver_sb: response[18].to_string().clone(),
                ver_layout: response[19].to_string().clone(),
                ultimo_cf_e_sat: response[20].to_string().clone(),
                lista_inicial: response[21].to_string().clone(),
                lista_final: response[22].to_string().clone(),
                dh_cfe: response[23].to_string().clone(),
                dh_ultima: response[24].to_string().clone(),
                cert_emissao: response[25].to_string().clone(),
                cert_vencimento: response[26].to_string().clone(),
                estado_operacao: response[27].to_string().clone(),
            }),
        }
    }
}

/* exemplo de resposta
"750726|                -> numero_sessao
10000|                  -> cod_retorno
Resposta com Sucesso.|  -> mensagem
|                       -> cod
|                       -> mensagemSEFAZ
001111111|              -> n_serie
DHCP|                   -> tipo_lan
000.000.000.000|        -> lan_ip
fc:52:ce:87:8e:27|      -> lan_mac
000.000.000.000|        -> lan_mask
000.000.000.000|        -> lan_gw
000.000.000.000|        -> lan_dns_1
000.000.000.000|        -> lan_dns_2
NAO_CONECTADO|          -> status_lan
ALTO|                   -> nivel_bateria
926016 Kbytes|          -> mt_total
11111 Kbytes|           -> mt_usada
20240722141324|         -> dh_atual
02.03.00|               -> ver_sb
0.08|                   -> ver_layout
35000000000000000000000000000000000000000000|   -> ultimo_cf_e_sat
00000000000000000000000000000000000000000000|   -> lista_inicial
00000000000000000000000000000000000000000000|   -> lista_final
20240719094846|                                 -> dh_cfe
20240719094846|                                 -> dh_ultima
20220101|                                        -> cert_emissao
20270101|                                       -> cert_vencimento
0"                                              -> estado_operacao

Estado de Operação do SAT
0= DESBLOQUEADO
1= BLOQUEIO SEFAZ
2= BLOQUEIO CONTRIBUINTE
3= BLOQUEIO AUTÔNOMO
4= BLOQUEIO PARA DESATIVAÇÃO
 */
