# Larrust

**Larrust** es una herramienta de desarrollo local ligera escrita en Rust que te permite gestionar fácilmente servicios como PHP, MySQL y phpMyAdmin desde la línea de comandos o una interfaz gráfica. Su objetivo es proporcionar una alternativa más rápida y eficiente a herramientas como XAMPP o Laragon, todo mientras aprovecha las ventajas de Rust.

## Requisitos

- **Rust**: Larrust está construido usando el lenguaje de programación Rust. Asegúrate de tenerlo instalado en tu máquina.
- **cargo**: Rust usa `cargo` como su gestor de paquetes y herramientas de compilación.

Puedes instalar Rust desde su sitio oficial: https://www.rust-lang.org/

## Instalación

1. **Clona este repositorio**:

    ```bash
    git clone https://github.com/VictorVasquezZT2005/larrust.git
    cd larrust
    ```

2. **Compila el proyecto**:

    Si no tienes Rust instalado, primero instala Rust y luego ejecuta el siguiente comando para compilar el proyecto:

    ```bash
    cargo build --release
    ```

3. **Agrega un alias para el comando**:

    Para facilitar el uso de `larrust`, agrega un alias a tu archivo de configuración de la terminal (`~/.bashrc` o `~/.zshrc`).

    ```bash
    alias larrust='cargo run --quiet --'
    source ~/.bashrc  # o source ~/.zshrc
    ```

## Comandos

### Iniciar servicios

1. **Iniciar servicios sin phpMyAdmin**:

    Si deseas iniciar Larrust sin phpMyAdmin, simplemente usa el siguiente comando:

    ```bash
    larrust start
    ```

2. **Iniciar con phpMyAdmin**:

    Para iniciar los servicios y también incluir phpMyAdmin, utiliza este comando:

    ```bash
    larrust start --with-phpmyadmin
    ```

### Detener servicios

Para detener los servicios de Larrust, usa el siguiente comando:

```bash
larrust stop
