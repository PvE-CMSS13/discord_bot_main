use crate::command::ExternalInternalError;

use serenity::all::Context;
use serenity::all::Message;

pub(super) async fn ping(ctx: &Context, msg: &Message) -> Result<(), ExternalInternalError> {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!".to_string()).await {
        return Err(ExternalInternalError {
            external_error: Some("Pong failed?".to_string()),
            internal_error: Some(format!("Ping: Error sending message: {why:?}").to_string()),
        });
    }

    return Ok(());
}
