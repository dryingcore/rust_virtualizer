use dotenv::dotenv;

pub fn load_env_variables() {
    dotenv().ok();
}
