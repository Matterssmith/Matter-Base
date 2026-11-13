# Matter Base

Base inicial del sistema Matter Bot.

## Tecnologías

- Rust
- WhatsApp Rust
- Tokio
- SQLite

## Instalación

Instalar Rust y Cargo.

Desde la raíz del proyecto ejecutar:

    cargo build

## Iniciar

    cargo run

## Conexión WhatsApp

La primera ejecución inicia el proceso de vinculación.

Escanea el código QR desde:

WhatsApp > Dispositivos vinculados > Vincular dispositivo.

La sesión se almacena en:

    data/whatsapp/session.db

No es necesario volver a vincular la cuenta después
de cada reinicio mientras la sesión siga siendo válida.

## Estado del bot

El bot viene desactivado por defecto.

Para habilitarlo, editar:

    config/bot.json

Cambiar:

    "enabled": false

Por:

    "enabled": true

Después reiniciar el programa.

## Pruebas

Enviar:

    #ping

Respuesta:

    Pong! Matter Base está conectado.

Enviar:

    #help

Muestra los comandos disponibles.

## Seguridad

No compartir la base de datos de sesión.

La carpeta data/ debe mantenerse fuera de repositorios públicos.
