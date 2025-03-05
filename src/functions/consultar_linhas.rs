use crate::config;
use crate::functions::generate_csv::write_to_csv;

use config::env_config::CONFIG;
use config::load_token::load_token;
use reqwest::Client;
use serde_json::Value;

pub async fn consultar_linhas(simcard_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = load_token().ok_or("Token não encontrado")?;

    let client = Client::new();
    let url = format!("{}{}", CONFIG.consultar_linha_url, simcard_id);
    let response = client.get(&url).bearer_auth(token).send().await?;

    let body: Value = response.json().await?;

    if body["status"] == 200 {
        let conteudo = &body["conteudo"];

        let full_caller_id = conteudo["fullCallerId"].as_str().unwrap_or("Desconhecido");
        let operadora = conteudo["operadora"].as_str().unwrap_or("Desconhecida");
        let saldo = conteudo["saldo"].as_f64().unwrap_or(0.0);
        let status_ativo = conteudo["statusAtivo"].as_str().unwrap_or("Indefinido");
        let iccid = conteudo["iccid"].as_str().unwrap_or("N/A");
        let plano_dados = conteudo["planoDadosMensal"].as_f64().unwrap_or(0.0);
        let data_ativacao = conteudo["dataAtivacao"].as_str().unwrap_or("Desconhecida");
        let imei_vinculado = conteudo["imei"].as_str().unwrap_or("Desconhecido");
        let nome_cliente = conteudo["descricaoCliente"]
            .as_str()
            .unwrap_or("Desconhecido");

        println!("NOME CLIENTE: {}", nome_cliente);
        println!("📞 Linha: {}", full_caller_id);
        println!("🏷️ Operadora: {}", operadora);
        println!("💰 Saldo: {:.2} MB", saldo);
        println!("📡 Status: {}", status_ativo);
        println!("📍 ICCID: {}", iccid);
        println!("📊 Plano de Dados Mensal: {:.0} MB", plano_dados);
        println!("🗓️ Data de Ativação: {}", data_ativacao);
        println!("IMEI Vinculado: {}", imei_vinculado);
        println!(
            "--------------------------------------------------------------------------------"
        );

        write_to_csv(
            nome_cliente,
            full_caller_id,
            operadora,
            saldo,
            status_ativo,
            iccid,
            plano_dados,
            data_ativacao,
            imei_vinculado,
        )?;
    } else {
        println!("❌ Erro ao consultar linhas: {:?}", body);
    }

    Ok(())
}
