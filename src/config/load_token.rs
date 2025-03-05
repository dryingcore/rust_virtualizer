use serde_json::Value;
use std::fs;

pub fn load_token() -> Option<String> {
    let data = fs::read_to_string("token.json").ok()?;
    let json: Value = serde_json::from_str(&data).ok()?;
    json["token"].as_str().map(String::from)
}
