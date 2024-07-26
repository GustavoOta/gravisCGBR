# gravisCGBR
API de comunicação com serviços do governo Brasileiro

## Configuração

Antes de iniciar a aplicação, você deve editar o arquivo `config.json` que esta no mesmo diretórios que o executável. Este arquivo deve conter os seguintes parâmetros:

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

#### Exemplo de Requisição

```http
GET http://localhost:3030/sat/consultar_sat