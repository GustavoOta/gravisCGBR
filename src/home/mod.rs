use actix_web::{get, HttpResponse, Responder};

#[get["/"]]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        "Gravis CGBR - Sistemas de Gestao Comercial ERP - CNPJ: 10.868.122/0001-58".to_string()
            + " - Versao: "
            + crate::config::version().as_str(),
    )
}
