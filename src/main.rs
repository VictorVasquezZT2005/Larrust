use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::Input;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};

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
            ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
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
        let content = fs::read_to_string("larrust.json")
            .context("No se pudo leer el archivo larrust.json")?;

        let config: Config = serde_json::from_str(&content)
            .context("Error al deserializar larrust.json")?;

        if config.web_server.is_empty() || config.database.is_empty() {
            anyhow::bail!("Faltan campos esenciales en larrust.json");
        }

        Ok(config)
    }
}

struct ProcessManager {
    phpmyadmin: Arc<Mutex<Option<Child>>>,
}

impl ProcessManager {
    fn new() -> Self {
        Self {
            phpmyadmin: Arc::new(Mutex::new(None)),
        }
    }

    fn start_phpmyadmin(&self, config: &PhpMyAdminConfig) -> Result<()> {
        let mut guard = self.phpmyadmin.lock().unwrap();

        if guard.is_some() {
            println!("phpMyAdmin ya está corriendo.");
            return Ok(());
        }

        if Command::new("php").arg("-v").output().is_err() {
            anyhow::bail!("PHP no está instalado o no está en PATH");
        }

        if !config.path.exists() {
            anyhow::bail!("Ruta phpMyAdmin no encontrada en {:?}", config.path);
        }

        let process = Command::new("php")
            .arg("-S")
            .arg(format!("{}:{}", config.ip, config.port))
            .arg("-t")
            .arg(&config.path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Error al iniciar phpMyAdmin")?;

        fs::write("phpmyadmin.pid", process.id().to_string())
            .context("No se pudo escribir phpmyadmin.pid")?;

        *guard = Some(process);
        println!("✅ phpMyAdmin corriendo en http://{}:{}", config.ip, config.port);
        Ok(())
    }

    fn stop_phpmyadmin(&self) -> Result<()> {
        let mut guard = self.phpmyadmin.lock().unwrap();

        if let Some(mut child) = guard.take() {
            let _ = child.kill();
            let _ = child.wait();
        }

        if let Ok(pid_str) = fs::read_to_string("phpmyadmin.pid") {
            if let Ok(pid) = pid_str.trim().parse::<u32>() {
                let _ = Command::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .status();
                println!("🛑 phpMyAdmin detenido (PID: {})", pid);
            }
            fs::remove_file("phpmyadmin.pid").ok();
        }

        Ok(())
    }
}

#[derive(Parser)]
#[command(name = "larrust", version = "0.1", about = "Un Laragon-like en Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long)]
        with_phpmyadmin: bool,
    },
    Stop,
    Create,
}

fn start_system_service(name: &str) -> Result<()> {
    Command::new("sudo")
        .arg("systemctl")
        .arg("start")
        .arg(name)
        .status()
        .context(format!("Error al iniciar el servicio {}", name))?;
    Ok(())
}

fn stop_system_service(name: &str) -> Result<()> {
    Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg(name)
        .status()
        .context(format!("Error al detener el servicio {}", name))?;
    Ok(())
}

fn create_php_project(name: &str) -> Result<()> {
    fs::create_dir_all(name)?;
    fs::write(format!("{}/index.php", name), "<?php phpinfo(); ?>")?;
    Ok(())
}

fn create_database(name: &str) -> Result<()> {
    Command::new("sudo")
        .arg("mysql")
        .arg("-e")
        .arg(format!("CREATE DATABASE IF NOT EXISTS `{}`;", name))
        .status()
        .context("Error al crear la base de datos")?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;
    let manager = ProcessManager::new();

    match cli.command {
        Commands::Start { with_phpmyadmin } => {
            start_system_service(&config.web_server)?;
            start_system_service(&config.database)?;

            if with_phpmyadmin && config.phpmyadmin.enabled {
                manager.start_phpmyadmin(&config.phpmyadmin)?;
            }

            println!("✅ Todos los servicios han sido iniciados.");
        }

        Commands::Stop => {
            manager.stop_phpmyadmin()?;
            stop_system_service(&config.web_server)?;
            stop_system_service(&config.database)?;
            println!("🛑 Todos los servicios han sido detenidos.");
        }

        Commands::Create => {
            let name: String = Input::new()
                .with_prompt("Nombre del nuevo proyecto")
                .interact_text()?;

            if PathBuf::from(&name).exists() {
                anyhow::bail!("El proyecto '{}' ya existe", name);
            }

            create_php_project(&name)?;

            match create_database(&name) {
                Ok(_) => println!("📦 Proyecto '{}' creado con su base de datos", name),
                Err(e) => {
                    println!("⚠️ Proyecto creado, pero falló la base de datos: {}", e);
                    println!("Puedes crearla manualmente con:");
                    println!("  sudo mysql -e \"CREATE DATABASE {};\"", name);
                }
            }
        }
    }

    Ok(())
}
