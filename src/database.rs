use anyhow::{Context, Result};
use which::which;
use std::process::{Command, Stdio};
use std::env;

pub fn start(service: &str) -> Result<()> {
    check_service_installed(service)?;

    let output = Command::new("sudo")
        .args(["systemctl", "start", service])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Error al iniciar {}", service))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Error al iniciar {}: {}\nSolución: sudo apt install {}",
            service,
            error_msg.trim(),
            suggested_package(service)
        );
    }
    Ok(())
}

pub fn stop(service: &str) -> Result<()> {
    check_service_installed(service)?;

    let output = Command::new("sudo")
        .args(["systemctl", "stop", service])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Error al detener {}", service))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Error al detener {}: {}", service, error_msg.trim());
    }
    Ok(())
}

pub fn restart(service: &str) -> Result<()> {
    stop(service)?;
    start(service)
}

pub fn create_database(name: &str) -> Result<()> {
    which("mysql").context("MySQL no está instalado. Instala con: sudo apt install mysql-server")?;

    let user = env::var("USER").unwrap_or_else(|_| "tu_usuario".to_string());

    let output = Command::new("mysql")
        .args(["-e", &format!("CREATE DATABASE IF NOT EXISTS {};", name)])
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => Command::new("sudo")
            .args(["mysql", "-e", &format!("CREATE DATABASE IF NOT EXISTS {};", name)])
            .output()
            .context("Error al crear la base de datos (incluso con sudo)")?
    };

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Error al crear DB: {}\nSolución:\n1. Configura acceso: sudo mysql -e \"CREATE USER '{}'@'localhost'; GRANT ALL PRIVILEGES ON *.* TO '{}'@'localhost';\"\n2. O crea manualmente: sudo mysql -e \"CREATE DATABASE {};\"",
            error_msg.trim(),
            user,
            user,
            name
        );
    }
    Ok(())
}

fn check_service_installed(service: &str) -> Result<()> {
    match which(service) {
        Ok(_) => Ok(()),
        Err(_) => anyhow::bail!(
            "{} no está instalado. Ejecuta: sudo apt install {}",
            service,
            suggested_package(service)
        )
    }
}

fn suggested_package(service: &str) -> &str {
    match service {
        "mysql" => "mysql-server",
        "mariadb" => "mariadb-server",
        _ => service
    }
}