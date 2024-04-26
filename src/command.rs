mod help;
mod link;
mod ping;

use crate::command::help::help;
use crate::command::link::link;
use crate::command::ping::ping;
use crate::DiscordHandler;

use serenity::all::Context;
use serenity::all::Message;

pub(super) struct ExternalInternalError {
    pub(super) external_error: Option<String>,
    pub(super) internal_error: Option<String>,
}

pub(super) async fn attempt_commands(
    handler: &DiscordHandler,
    ctx: &Context,
    msg: &Message,
) -> Result<(), ExternalInternalError> {
    let message_split: Vec<&str> = msg.content.split(&[' ']).collect();

    let command_string;

    match message_split.get(0) {
        Some(good_string) => {
            command_string = *good_string;
        }
        None => {
            return Ok(());
        }
    }

    match command_string {
        "!help" => {
            return help(ctx, msg).await;
        }
        "!link" => {
            return link(handler, ctx, msg).await;
        }
        "!ping" => {
            return ping(ctx, msg).await;
        }
        _ => {
            return Ok(());
        }
    }
}
