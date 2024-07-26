use crate::migrations;
pub async fn init() {
    // ********************* SYSTEM UPDATE ********************* //
    // auto update system

    // ********************* DATABASE MIGRATIONS ********************* //
    // create update delete database tables and columns
    migrations::exec().await;

    // ******** VERIFICAÇÃO DE ESTRUTURA DE DIRETORIOS ********* //
    // mkdirs

    // ******** VERIFICAÇÃO DE ARQUIVOS ********* //
    // check files
    crate::config::check_file().await;

    // ********************* TASKS ********************* //
    let _task_name = tokio::spawn(async move {
        loop {
            let timeout = 1000;
            tokio::time::sleep(std::time::Duration::from_secs(timeout)).await;
        }
    });
}
