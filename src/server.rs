use anyhow::{Context, Result};
use which::which;
use std::process::{Command, Stdio};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn start(service: &str) -> Result<()> {
    match service {
        "phpmyadmin" => start_phpmyadmin(),
        _ => {
            which(service).with_context(|| format!("{} no está instalado", service))?;

            let output = Command::new("sudo")
                .args(["systemctl", "start", service])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .with_context(|| format!("Error ejecutando systemctl start {}", service))?;

            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Error al iniciar {}: {}", service, error_msg.trim());
            }
            Ok(())
        }
    }
}

pub fn stop(service: &str) -> Result<()> {
    match service {
        "phpmyadmin" => stop_phpmyadmin(),
        _ => {
            which(service).with_context(|| format!("{} no está instalado", service))?;

            let output = Command::new("sudo")
                .args(["systemctl", "stop", service])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .with_context(|| format!("Error ejecutando systemctl stop {}", service))?;

            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                anyhow::bail!("Error al detener {}: {}", service, error_msg.trim());
            }
            Ok(())
        }
    }
}

pub fn restart(service: &str) -> Result<()> {
    match service {
        "phpmyadmin" => {
            stop_phpmyadmin()?;
            start_phpmyadmin()
        },
        _ => {
            stop(service)?;
            start(service)
        }
    }
}

// Funciones específicas para phpMyAdmin
static mut PHPMA_PROCESS: Option<std::process::Child> = None;

fn start_phpmyadmin() -> Result<()> {
    // Verificar si php está instalado
    which("php").with_context(|| "PHP no está instalado. Necesario para phpMyAdmin")?;

    // Verificar que existe la carpeta phpmyadmin
    if !Path::new("phpmyadmin").exists() {
        anyhow::bail!("Carpeta phpmyadmin no encontrada en el directorio raíz");
    }

    // Verificar que contiene los archivos necesarios
    if !Path::new("phpmyadmin/index.php").exists() {
        anyhow::bail!("La carpeta phpmyadmin no contiene los archivos correctos");
    }

    unsafe {
        if PHPMA_PROCESS.is_none() {
            let process = Command::new("php")
                .arg("-S")
                .arg("localhost:8081")
                .arg("-t")
                .arg("phpmyadmin")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .with_context(|| "Error al iniciar phpMyAdmin")?;

            PHPMA_PROCESS = Some(process);
            thread::sleep(Duration::from_secs(1));
            println!("✅ phpMyAdmin iniciado en http://localhost:8081");
        } else {
            println!("ℹ️ phpMyAdmin ya está en ejecución");
        }
    }
    Ok(())
}

fn stop_phpmyadmin() -> Result<()> {
    unsafe {
        if let Some(mut process) = PHPMA_PROCESS.take() {
            process.kill().with_context(|| "Error al detener phpMyAdmin")?;
            println!("🛑 phpMyAdmin detenido");
        } else {
            println!("ℹ️ phpMyAdmin no estaba en ejecución");
        }
    }
    Ok(())
}