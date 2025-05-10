use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn create_project(name: &str) -> Result<()> {
    let project_path = Path::new(name);
    
    if project_path.exists() {
        anyhow::bail!("El proyecto '{}' ya existe", name);
    }

    fs::create_dir(project_path)
        .context("No se pudo crear la carpeta del proyecto")?;

    let index_php = r#"<?php 
echo "¡Hola desde LarRust!";
phpinfo();
?>"#;

    fs::write(project_path.join("index.php"), index_php)
        .context("No se pudo crear index.php")?;

    Ok(())
}