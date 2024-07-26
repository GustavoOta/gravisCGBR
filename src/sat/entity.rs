use serde::{Deserialize, Serialize};

// *************** ConsultarSat ***************
// 266122|08000|SAT em operaÃ§Ã£o||

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqConsultarSat {
    pub codigo_sessao: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RespConsultarSat {
    pub error: u8,
    pub code: Option<i32>,
    pub msg: String,
    pub data: Option<ConsultarSatData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsultarSatData {
    pub numero_sessao: i32,
    pub cod_retorno: i32,
    pub mensagem: String,
}
// *********************************************
// *************** ConsultarStatusOperacional ***************

#[derive(Debug, Serialize, Deserialize)]
pub struct RespConsultarStatusOperacional {
    pub error: u8,
    pub code: Option<i32>,
    pub msg: String,
    pub data: Option<ConsultarStatusOperacionalData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsultarStatusOperacionalData {
    pub numero_sessao: i32,
    pub cod_retorno: i32,
    pub mensagem: String,
    pub cod: String,
    pub mensagem_sefaz: String,
    pub n_serie: String,
    pub tipo_lan: String,
    pub lan_ip: String,
    pub lan_mac: String,
    pub lan_mask: String,
    pub lan_gw: String,
    pub lan_dns_1: String,
    pub lan_dns_2: String,
    pub status_lan: String,
    pub nivel_bateria: String,
    pub mt_total: String,
    pub mt_usada: String,
    pub dh_atual: String,
    pub ver_sb: String,
    pub ver_layout: String,
    pub ultimo_cf_e_sat: String,
    pub lista_inicial: String,
    pub lista_final: String,
    pub dh_cfe: String,
    pub dh_ultima: String,
    pub cert_emissao: String,
    pub cert_vencimento: String,
    pub estado_operacao: String,
}

// *********************************************
/*
exemplo de consultar status operacional

exemplo de resposta
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
