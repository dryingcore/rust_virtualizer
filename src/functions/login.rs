use crate::config::env_config::CONFIG;
use reqwest::Client;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;

pub async fn login_request() -> Result<String, Box<dyn std::error::Error>> {
    let login_url = &CONFIG.login_url;
    let email = &CONFIG.email;
    let password = &CONFIG.password;

    let client = Client::new();
    let response = client
        .post(login_url)
        .json(&json!({
            "login": email,
            "senha": password
        }))
        .send()
        .await?;

    let response_text = response.text().await?;

    let json_response: Value = serde_json::from_str(&response_text)?;

    if let Some(token) = json_response["conteudo"]["token"].as_str() {
        save_token_to_file(token)?;
        println!("Token salvo com sucesso.");
    } else {
        println!("Token não encontrado na resposta.");
    }
    Ok(response_text)
}

fn save_token_to_file(token: &str) -> std::io::Result<()> {
    let mut file = File::create("token.json")?;
    let json_data = json!({ "token": token }).to_string();
    file.write_all(json_data.as_bytes())?;
    Ok(())
}
