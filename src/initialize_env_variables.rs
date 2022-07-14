use dotenv::dotenv;

pub fn initialize_env_variables() {
    dotenv().ok();
}
