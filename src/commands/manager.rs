use tracing::warn;
use whatsapp_rust::bot::MessageContext;

pub async fn execute(
    ctx: MessageContext,
    command: String,
) {
    match command.as_str() {
        "ping" => {
            if let Err(error) = ctx
                .reply("Pong! Matter Bot está conectado.")
                .await
            {
                warn!("No se pudo responder a ping: {}", error);
            }
        }

        "help" => {
            let response = "\
╭━━〔 MATTER BOT 〕
┃
┃ Comandos disponibles:
┃
┃ #ping
┃ #help
┃
╰━━━━━━━━━━━━";

            if let Err(error) = ctx.reply(response).await {
                warn!("No se pudo responder a help: {}", error);
            }
        }

        _ => {}
    }
}
