use once_cell::sync::Lazy;
use std::env;

pub struct EnvConfig {
    pub login_url: String,
    pub consultar_linha_url: String,
    pub email: String,
    pub password: String,
    pub resetar_linha_url: String,
}

pub static CONFIG: Lazy<EnvConfig> = Lazy::new(|| EnvConfig {
    login_url: env::var("LOGIN_URL").expect("❌ ERRO: LOGIN_URL não está definida"),
    consultar_linha_url: env::var("CONSULTAR_LINHA_URL")
        .expect("❌ ERRO: CONSULTAR_LINHA_URL não está definida"),
    email: env::var("EMAIL").expect("❌ ERRO: EMAIL não está definida"),
    password: env::var("PASSWORD").expect("❌ ERRO: PASSWORD não está definida"),
    resetar_linha_url: env::var("RESETAR_LINHA_URL").expect(
        "❌ ERRO: RESETAR_LINHA_URL não está \
    definida",
    ),
});
