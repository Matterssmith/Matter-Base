use whatsapp_rust::bot::MessageContext;

use crate::commands;

pub async fn dispatch(
    ctx: MessageContext,
    message: String,
    prefix: String,
) {
    let message = message.trim();

    let Some(command_line) = message.strip_prefix(&prefix) else {
        return;
    };

    let mut parts = command_line.split_whitespace();

    let Some(command) = parts.next() else {
        return;
    };

    let command = command.to_ascii_lowercase();

    commands::manager::execute(ctx, command).await;
}
