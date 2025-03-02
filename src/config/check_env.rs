use std::env;

pub fn check_env_variables() {
    let required_vars = [
        ("LOGIN_URL", "🔗"),
        ("CONSULTAR_LINHA_URL", "🔗"),
        ("EMAIL", "📧"),
        ("PASSWORD", "🔑"),
    ];

    for (var, icon) in &required_vars {
        match env::var(var) {
            Ok(_) => println!("✅ {} {} está definido.", icon, var),
            Err(_) => panic!("❌ {} {} não está definido!", icon, var),
        }
    }
}
