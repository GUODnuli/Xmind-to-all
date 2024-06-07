use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct ConfigToml {
    user_config: HashMap<String, String>,
}

// 公有的单例 user_config 对象
pub static USER_CONFIG: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| {
    let config_path = Path::new("config/config.toml");
    let config_toml_content = fs::read_to_string(config_path).expect("Failed to read config.toml");
    let config_toml: ConfigToml = toml::from_str(&config_toml_content).expect("Failed to parse Cargo.toml");

    Arc::new(Mutex::new(config_toml.user_config))
});

pub fn get_user_config(user_config_data: Arc<Mutex<HashMap<String, String>>>) -> HashMap<String, String> {
    let config = user_config_data.lock().unwrap();
    config.clone()
}

pub fn print_user_config(user_config_data: Arc<Mutex<HashMap<String, String>>>) {
    let config = user_config_data.lock().unwrap();
    for (key, value) in config.iter() {
        println!("{}: {}", key, value);
    }
}