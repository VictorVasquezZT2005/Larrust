use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhpMyAdminConfig {
    pub enabled: bool,
    pub port: u16,
    pub ip: IpAddr,
    pub path: PathBuf,
}

impl Default for PhpMyAdminConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            port: 8081,
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            path: PathBuf::from("phpmyadmin"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub web_server: String,
    pub database: String,
    pub php_version: String,
    #[serde(default)]
    pub phpmyadmin: PhpMyAdminConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_file = fs::read_to_string("larrust.json")
            .context("No se pudo leer larrust.json")?;
        
        let mut config: Config = serde_json::from_str(&config_file)
            .context("Error al parsear larrust.json")?;
        
        if config.web_server.is_empty() || config.database.is_empty() {
            anyhow::bail!("Configuración inválida en larrust.json");
        }

        Ok(config)
    }
}