# Matter Base

Base inicial del ecosistema Matterssmith.

## Tecnologías

- Rust
- Tokio
- WhatsApp Rust
- SQLite

## Requisitos

- Rust
- Cargo
- Conexión a Internet
- Una cuenta de WhatsApp activa

## Configuración

Editar:

    config/bot.json

Configurar el número internacional de WhatsApp:

    "phone_number": "52XXXXXXXXXX"

El número debe incluir el código de país.

Se permiten únicamente los caracteres de formato habituales.
El sistema normaliza el número antes de enviarlo al proveedor.

## Instalación

Desde la raíz:

    cargo build

## Iniciar

    cargo run

## Vincular WhatsApp

Matter Base utiliza el emparejamiento mediante número de teléfono.

1. Iniciar Matter Base en el servidor.
2. Esperar el código de ocho caracteres.
3. Abrir WhatsApp en el teléfono de la cuenta que se vinculará.
4. Entrar en Dispositivos vinculados.
5. Seleccionar Vincular dispositivo.
6. Seleccionar Vincular con número de teléfono.
7. Introducir el código mostrado por Matter Base.

No es necesario escanear el QR.

## Sesión

La sesión se almacena en:

    data/whatsapp/session.db

La sesión persiste entre reinicios mientras WhatsApp
mantenga el dispositivo vinculado.

No publicar ni compartir este archivo.

## Configuración del bot

Por defecto:

    "enabled": false

Para habilitar:

    "enabled": true

Prefijo:

    "#"

## Comandos

    #ping
    #help

## Arquitectura

Core
  -> Provider Manager
  -> Endpoint Manager
  -> WhatsApp
  -> Router
  -> Command Manager

## Seguridad

La conexión utiliza una implementación comunitaria
del protocolo de WhatsApp Web.

No es una integración oficial de Meta.
Utilizar clientes alternativos puede implicar restricciones
sobre la cuenta de WhatsApp.
