# gravisCGBR
API de comunicação com serviços do governo Brasileiro

## Configuração

Antes de iniciar a aplicação, você deve editar o arquivo `config.json` que esta no mesmo diretório que o executável, o executável cria o arquivo ao ser executado, ou crie manualmente. Este arquivo deve conter os seguintes parâmetros:

```json
{
    "server": {
        "server_host": "0.0.0.0",
        "server_port": 3030,
        "version": "1.0.1"
    },
    "sat": {
        "cnpj": "11111111111111",
        "sign_ac": "AC_SIGNATURE",
        "codigo_ativacao": "00000000",
        "dll_path": "C:/sat.dll",
        "dll_load_enviroment": "win64",
        "dll_convention": "stdcall",
        "dll_maker": "DEMO"
    }
}
```
### Detalhes dos Campos

#### server:

- **server_host**: O endereço IP onde o servidor será executado. Exemplo: `"0.0.0.0"`.
- **server_port**: A porta na qual o servidor ouvirá. Exemplo: `3030`.
- **version**: A versão da API. Exemplo: `"1.0.1"`.

#### sat:

- **cnpj**: O CNPJ da empresa. Exemplo: `"11111111111111"`.
- **sign_ac**: A assinatura AC. Exemplo: `"AC_SIGNATURE"`.
- **codigo_ativacao**: O código de ativação do SAT. Exemplo: `"00000000"`.
- **dll_path**: O caminho para a DLL do SAT. Exemplo: `"C:/sat.dll"`.
- **dll_load_enviroment**: O ambiente de carregamento da DLL. Exemplo: `"win64"`.
- **dll_convention**: A convenção de chamada da DLL. Exemplo: `"stdcall"`.
- **dll_maker**: O fabricante da DLL. Exemplo: `"DEMO"`.

## Endpoints

### Consultar Status do SAT

- **URL**: `http://localhost:3030/sat/consultar_sat`
- **Método**: `GET`
- **Descrição**: Este endpoint consulta o status do equipamento SAT.

#### Exemplo de Resposta
```json
{
    "error": 0,
    "code": 0,
    "msg": "Sucesso ao consultar equipamento SAT",
    "data": {
            "numero_sessao": 473295,
            "cod_retorno": 8000,
            "mensagem": "SAT em operação"
            }
}
```
- **URL**: `http://localhost:3030/sat/consultar_sat`
- **Método**: `GET`
- **Descrição**: Este endpoint consulta o status do equipamento SAT.

#### Exemplo de Resposta
```json
{
    "error": 0,
    "code": 0,
    "msg": "Sucesso ao consultar equipamento SAT",
    "data": {
        "numero_sessao": 473295,
        "cod_retorno": 8000,
        "mensagem": "SAT em operação"
    }
}
```

- **URL**: `http://localhost:3030/sat/consultar_status_operacional`
- **Método**: `GET`
- **Descrição**: Este endpoint consulta o status operacional do equipamento SAT.

#### Exemplo de Resposta
```json
{
"error": 0,
"code": 0,
"msg": "Sucesso ao consultar o equipamento SAT",
"data": {
    "numero_sessao": 546320,
    "cod_retorno": 10000,
    "mensagem": "Resposta com Sucesso.",
    "cod": "",
    "mensagem_sefaz": "",
    "n_serie": "001200000",
    "tipo_lan": "DHCP",
    "lan_ip": "192.168.001.064",
    "lan_mac": "xx:00:xx:00:xx:00",
    "lan_mask": "255.255.255.000",
    "lan_gw": "192.168.001.254",
    "lan_dns_1": "000.000.000.000",
    "lan_dns_2": "000.000.000.000",
    "status_lan": "CONECTADO",
    "nivel_bateria": "ALTO",
    "mt_total": "900000 Kbytes",
    "mt_usada": "1 Kbytes",
    "dh_atual": "20240726132332",
    "ver_sb": "02.03.00",
    "ver_layout": "0.08",
    "ultimo_cf_e_sat": "35240000000000000000000000000000000000000000",
    "lista_inicial": "00000000000000000000000000000000000000000000",
    "lista_final": "00000000000000000000000000000000000000000000",
    "dh_cfe": "20240726000000",
    "dh_ultima": "20240726000000",
    "cert_emissao": "20220000",
    "cert_vencimento": "20270000",
    "estado_operacao": "0"
    }
}
```

