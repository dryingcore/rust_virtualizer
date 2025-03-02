mod config;
mod functions;

use inquire::Select;
use config::check_env::check_env_variables;
use config::load_env::load_env_variables;
use crate::functions::consultar_linhas::consultar_linhas;
use crate::functions::login::login_request;

#[tokio::main]
async fn main() {
    load_env_variables();
    check_env_variables();

    match login_request().await {
        Ok(_) => println!("Login realizado com sucesso!"),
        Err(error) => {
            println!("Erro ao realizar login: {}", error);
            return;
        }
    }

    loop {
        let options = vec![
            "Consultar Linhas",
            "Gerar Relatório",
            "Bloquear Linhas",
            "Desbloquear Linhas",
            "Ver informações do cliente",
            "Sair",
        ];

        let choice = Select::new("Escolha uma opção:", options).prompt();

        match choice {
            Ok("Consultar Linhas") => {
                if let Err(e) = consultar_linhas("89555480000040720043").await {
                    println!("Erro ao consultar linhas: {}", e);
                }
            },
            Ok("Sair") | _ => {
                println!("👋 Saindo...");
                break;
            }
        }
    }
}