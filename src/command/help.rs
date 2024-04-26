use crate::command::ExternalInternalError;

use serenity::all::Context;
use serenity::all::Message;

pub(super) async fn help(ctx: &Context, msg: &Message) -> Result<(), ExternalInternalError> {
    let response = format!("
		!help - Outputs this text.\n\
		!link [identifier] - Links your discord and byond account. This gives you access to \"prime\" priority in game for PltCo/PltSgt. Get an identifier via Discord Certify in the OOC tab on the server.\n\
		!ping - Pong!
	");

    if let Err(why) = msg.channel_id.say(&ctx.http, response.to_string()).await {
        return Err(ExternalInternalError {
            external_error: Some("Help failed?".to_string()),
            internal_error: Some(format!("Help: Error sending message: {why:?}").to_string()),
        });
    }

    return Ok(());
}
