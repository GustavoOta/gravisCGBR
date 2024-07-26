use actix_web::{get, HttpResponse, Responder};

mod common;
mod consultar_sat;
mod consultar_status_operacional;
mod entity;
//use common::*;
use consultar_sat::*;
use consultar_status_operacional::*;
//use entity::*;

#[get("/sat/consultar_sat")]
pub async fn consultar_sat_index() -> impl Responder {
    let response = exec_consultar_sat().await;
    let convert_to_json = json_consultar_sat(response).await;
    HttpResponse::Ok().json(convert_to_json)
}

#[get("/sat/consultar_status_operacional")]
pub async fn consultar_status_operacional_index() -> impl Responder {
    let response = exec_consultar_status_operacional().await;
    println!("response: {:?}", response);
    let convert_to_json = json_consultar_status_operacional(response).await;
    HttpResponse::Ok().json(convert_to_json)
}

/*
instruções para o arquivo src/sat/mod.rs
Parâmetros de entrada das funções:
Para chamar as funções e passar os dados necessários existe um conjunto de parâmetros para cada função.
Mesmo que cada função tenha uma tarefa diferente, alguns parâmetros são comuns a todas elas e podem
ser tratados de forma padronizada. Segue os parâmetros comuns as funções.
Descrição dos Parâmetros comuns da função
Parâmetro Descrição Tipo Limite Exemplo
numeroSessao Número aleatório gerado pelo
AC para controle da
comunicação
Inteiro Entre 0 e 999999 Ex: 1, 2101 ,
123456
codigoDeAtivacao Senha definida pelo
contribuinte no software de
ativação.
String Alfanumérico.
Tendo entre 8 a 32
caracteres.
Ex:
“bG9fZGaWdvX”
Ex:“senha123456”
4.2.1. Tratamento do parâmetro “numeroSessao”.
Este parâmetro é um inteiro entre 0 e 999999 que deve ser diferente para cada chamada de qualquer
função do SAT, não podendo ser repetido pelas ultimas 100 chamadas.
Este comportamento de não repetição tem por mérito permitir o uso do comando ConsultarNumeroSessao
e ConsultarUltimaSessaoFiscal, para que seja possível validar se o retornos dessas funções são realmente o
esperado. (Vide capitulo 4.4.7 ConsultarNumeroSessao e capitulo 4.4.8 ConsultarUltimaSessaoFiscal)
4.2.2. Tratamento do parâmetro “codigoDeAtivacao”.
Este parâmetro é do tipo texto (tipo const char* em C\C++, String Java) devendo ter entre 8 e 32
caracteres.
O código de ativação tem a função de senha de acesso do equipamento e é cadastrada durante a ativação
do SAT (AtivarSAT), sendo possível alterá-la a partir da função TrocarCodigoDeAtivacao.
Caso se informe três vezes seguidas o código de ativação errado o SAT ira bloquear por um tempo definido.
Recomendamos validar se a função falhou por conta de código de ativação errado e avisar o usuário antes
que o SAT bloqueie.
Manual de instalação v.3
SAT Sweda
‘ 7
4.2.3. Tratamento de outros tipos de parâmetro.
As funções do SAT possuem apenas 2 tipos de dados na entrada, Inteiro (unsigned int para C\C++) ou texto
(tipo const char* em C\C++).
A passagem desses dados costuma ter tratativas próprias para cada linguagem de programação,
normalmente atrelado a forma que se é carregado a DLL/biblioteca no software.
Alguns parâmetros esperam dados do tipo texto contento um XML, nesse caso o XML não deverá conter o
caractere pipe “|” entre suas tags.
O padrão de codificação dos caracteres para os campos do tipo texto é “UTF-8”.

4.4.4. ConsultarSAT
Esta função faz uma comunicação simples com o equipamento a fim de avaliar se o mesmo esta se
comunicando corretamente.
Chamada da função.
Assinatura da função (C \ C++):
const char* ConsultarSAT(int numeroSessao)
Assinatura da função (Java):
String ConsultarSAT (int numeroSessao)
Descrição dos Parâmetros da Função
Parâmetro Descrição Tipo Limite Exemplo
numeroSessao Número aleatório
gerado pelo AC para
controle da comunicação
Vide 4.2.1
Inteiro Entre 0 e 999999 Ex: 1, 2101 , 123456
Retorno da Função:
Estrutura de Retorno:
“numeroSessao|EEEEE|mensagem|cod|mensagemSEFAZ”
Importante. Os parâmetros de retornos são os comuns usados por todas as funções (ver 4.3).
Tabelas de resultados (EEEEE):
Código de
Retorno
Mensagem
8000 SAT em operação.
8098 SAT em processamento. Tente novamente.
8099 Erro desconhecido.

4.4.5. ConsultarStatusOperacional
Esta função executa uma consulta ao SAT para trazer status, dados de configuração, o ultimo CFE
emitido e outros dados úteis.
Esta função não depende de conexão da internet para funcionar.
Pode ser chamada a qualquer momento no SAT,
Se o SAT estiver desativado o parâmetro codigoDeAtivacao deve ser “00000000”.
Chamada da função.
Assinatura da função (C \ C++):
const char* ConsultarStatusOperacional(int numeroSessao, const char* codigoDeAtivacao)
Assinatura da função (Java):
String ConsultarStatusOperacional(int numeroSessao, String codigoDeAtivacao)
Importante. Os únicos parâmetros são os comuns usados por todas as funções (ver 4.2).
Retorno da Função:
Estrutura de Retorno com SUCESSO (EEEEE = 10000):
“numeroSessao|EEEEE|mensagem|cod|mensagemSEFAZ|ConteudoDoRetorno”
Estrutura de Retorno com ERRO (EEEEE != “10000”):
“numeroSessao|EEEEE|mensagem|cod|mensagemSEFAZ”
Importante. Os 5 primeiros parâmetros de retornos são os comuns usados por todas as funções
(ver 4.3).
No caso de sucesso, após os 5 primeiros parâmetros começa o ConteudoDoRetorno, esse é uma
lista com 22 parâmetros que estão descritos na tabela abaixo.
Descrição do ConteudoDoRetorno da Função no caso de sucesso
Nome Campo Tamanho
Máximo
Exemplo Descrição
NSERIE 9 320008889 Número de série do SAT
TIPO_LAN 8 DHCP, PPPoE, IPFIX Tipo de Lan
LAN_IP 15 192.168.010.100 Endereço IP da LAN
LAN_MAC 17 00:0C:41:82:25:E8 Endereço MAC
LAN_MASK 15 255.255.255.000 Máscara de sub-rede
LAN_GW 15 192.168.010.001 Endereço gateway
LAN_DNS_1 15 192.168.010.001 Endereço DNS1
LAN_DNS_2 15 192.168.010.001 Endereço DNS2
STATUS_LAN 16 CONECTADO Status da rede
NIVEL_BATERIA 8 ALTO, MÉDIO, BAIXO Nível da Bateria
MT_TOTAL - 1 Gbyte Memória de Trabalho Total
MT_USADA - 35 bytes Memória de Trabalho Usada
Manual de instalação v.3
SAT Sweda
‘ 21
Os dados presentes na tabela podem ser usados para lógicas internas de monitoramento ou
tomada de decisão, segue os dados com mais relevância:
 NSERIE: Numero de serie do equipamento.
 Pode ser utilizado para manter o registro de qual equipamento esta trabalhando no
PDV e permitir aplicar comportamentos específicos para cada SAT.
 STATUS_LAN: Indica se o SAT tem comunicação com a internet.
 VER_LAYOUT: versão do layout de comunicação com a SEFAZ.
 Não usar este parâmetro diretamente no campo
 ULTIMO_CF-E-SAT: Chave do ultimo CF-e-SAT emitido pelo equipamento (venda ou cancelamento)
 ESTADO_OPERACAO: Numero que indica o estado do equipamento. Para conseguir executar
vendas deve estar no estado 0.
 LISTA_ INICIAL e LISTA_ FINAL: Indica o primeiro e ultimo cupom da lista de cupom presentes na
memória do SAT.
Tabelas de resultados (EEEEE):
Código de
Retorno
Mensagem
10000 Resposta com Sucesso.
10001 Código de ativação inválido
10098 SAT em processamento. Tente novamente.
10099 Erro desconhecido
DH_ATUAL 14 20111021170022 Data e hora atual no formato
AAAAMMDDhhmmss
VER_LAYOUT 16 1.01 Versão do Leiaute da tabela de
informações
ULTIMO_CF-E-SAT 44 32008889000000089 Número sequencial do Último CFe-SAT Emitido
LISTA_ INICIAL 44 32008889000000075 Número sequencial do primeiro
CF-e-SAT armazenado na
memória de trabalho
LISTA_ FINAL 44 32008889000000089 Número sequencial do último CFe-SAT armazenado na memória
de trabalho
DH_CFe 14 20111021170022 Data e hora da última transmissão
de CF-e-SAT para SEFAZ no
formato AAAAMMDDhhmmss
DH_ULTIMA 14 20111021170022 Última comunicação com a SEFAZ
no formato AAAAMMDDhhmmss
CERT_EMISSAO 08 2011102

*/
