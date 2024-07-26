use rand::Rng;

pub fn cod_sessao() -> i32 {
    // retorna um número aleatório entre 100000 e 999999
    // para ser usado como código de sessão e não repetir nas últimas 100 chamadas
    let mut rng = rand::thread_rng();
    rng.gen_range(100000..999999)
}

pub fn _validar_cod_sessao(cod_sessao: Option<i32>) -> bool {
    // valida se o número de sessão é válido
    // e se não foi repetido nas últimas 100 chamadas
    if cod_sessao.is_none() {
        return false;
    }
    let cod_sessao = cod_sessao.unwrap();
    if cod_sessao < 100000 || cod_sessao > 999999 {
        return false;
    }
    true
}
