# Template para proyecto web con Rust + actix_web + tera + diesel 

{{project-description}} 

## Author: {{author}}

# Features

- Conexión de ejemplo a servidor Postgres en container Docker

`docker run -d --name rustpg -e POSTGRES_PASSWORD=secret -e POSTGRES_USER=rustuser -e POSTGRES_DB=rustdb -p 5432:5432 postgres
`

- Módulo utils con funciones de conexión a la DDBB y acceso a archivos estáticos
- Primera migración vacía, para creación de tablas en la DDBB

`
diesel migration run
`

- Carpeta para models
- Carpeta para views
- Carpeta templates
- Página Home de base
