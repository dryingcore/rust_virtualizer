use crate::config::env_config::CONFIG;
use crate::config::load_token::load_token;
use reqwest::Client;

pub async fn resetar_linhas(simcard_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = load_token().ok_or_else(|| "Token não encontrado")?;
    let client = Client::new();
    let url = CONFIG.resetar_linha_url.replace("{}", simcard_id);

    let response = client.put(&url).bearer_auth(token).send().await?;

    if response.status().is_success() {
        println!("Reset feito com sucesso!!!");
        Ok(())
    } else {
        Err(Box::from("Ocorreu"))
    }
}
