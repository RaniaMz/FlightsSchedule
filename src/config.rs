use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server_addr: String,
}

pub fn get_config() -> Config {
    dotenv::dotenv().ok();
    envy::from_env::<Config>().expect("Config error")
}