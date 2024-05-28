use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
    dependencies: Option<toml::Value>,
    user_config: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

// 公有的单例 user_config 对象
pub static USER_CONFIG: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let cargo_toml_content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let cargo_toml: CargoToml = toml::from_str(&cargo_toml_content).expect("Failed to parse Cargo.toml");

    Mutex::new(cargo_toml.user_config)
});