# Para poder usar el comando larrust
alias larrust='cargo run --quiet --'
source ~/.bashrc  # o source ~/.zshrc

# Iniciar servicios (sin phpMyAdmin)
larrust start

# Iniciar con phpMyAdmin
larrust start --with-phpmyadmin

# Detener servicios
larrust stop

# Crear proyecto (te preguntará el nombre)
larrust create