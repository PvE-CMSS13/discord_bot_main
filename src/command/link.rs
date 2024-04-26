use crate::command::ExternalInternalError;
use crate::helper::convert_to_safe_message;
use crate::DiscordHandler;

use serenity::all::Context;
use serenity::all::Message;

use sqlx::Error;
use sqlx::Row;

pub(super) async fn link(
    handler: &DiscordHandler,
    ctx: &Context,
    msg: &Message,
) -> Result<(), ExternalInternalError> {
    if let Err(error) = msg.delete(&ctx.http).await {
        return Err(ExternalInternalError {
            external_error: Some("Permissions error. Contact admin.".to_string()),
            internal_error: Some(format!("Link: Error deleting message: {error:?}").to_string()),
        });
    }

    let database_connection = match handler.database.clone() {
        Some(url) => url,
        None => {
            return Err(ExternalInternalError {
                external_error: Some("Database features not enabled.".to_string()),
                internal_error: None,
            })
        }
    };

    let message_split: Vec<&str> = msg
        .content
        .split(&[' ', ';', '*', '(', ')', '\'', '!'])
        .collect();

    if message_split.len() != 3 {
        return Err( ExternalInternalError { external_error: Some("Usage: \"!link [server identifier]\"\nGet an identifier via Discord Certify in the OOC tab on the server.".to_string()), internal_error: None } );
    }

    let discord_id = msg.author.id.to_string();

    let discord_link_query_result = sqlx::query("SELECT * FROM discord_links WHERE discord_id = ?")
        .bind(discord_id.clone())
        .fetch_one(&database_connection)
        .await;

    match discord_link_query_result {
        Ok(_) => {
            return Err( ExternalInternalError { external_error: Some("This discord account is already linked. Reach out to an administrator if this is incorrect.".to_string()), internal_error: None } );
        }
        Err(error) => match error {
            Error::RowNotFound => {}
            _ => {
                return Err(ExternalInternalError {
                    external_error: Some("Query error. Contact admin.".to_string()),
                    internal_error: Some(
                        format!("Link: Error querying discord_links: {error:?}").to_string(),
                    ),
                });
            }
        },
    }

    let identifier;

    match message_split.get(2) {
        Some(x) => identifier = x,
        None => {
            return Err(ExternalInternalError {
                external_error: Some("Given identifier not found.".to_string()),
                internal_error: None,
            });
        }
    };

    let discord_identifier_query_result =
        sqlx::query("SELECT * FROM discord_identifiers WHERE identifier = ?")
            .bind(identifier)
            .fetch_one(&database_connection)
            .await;

    let discord_identifier_fetched_row;

    match discord_identifier_query_result {
        Ok(good_row) => {
            discord_identifier_fetched_row = good_row;
        }
        Err(error) => match error {
            Error::RowNotFound => {
                return Err( ExternalInternalError { external_error: Some("Unable to find identifier. Recheck you are copying the code directly from the in game menu.".to_string()), internal_error: None } );
            }
            _ => {
                return Err(ExternalInternalError {
                    external_error: Some("Query error. Contact admin.".to_string()),
                    internal_error: Some(
                        format!("Link: Error getting row from discord_identifiers: {error:?}")
                            .to_string(),
                    ),
                });
            }
        },
    }

    let used: i64;

    match discord_identifier_fetched_row.try_get("used") {
        Ok(good_used) => {
            used = good_used;
        }
        Err(error) => {
            return Err(ExternalInternalError {
                external_error: Some("Query error. Contact admin.".to_string()),
                internal_error: Some(
                    format!("Link: Error getting used from discord_identifiers row: {error:?}")
                        .to_string(),
                ),
            });
        }
    }

    //Not worth converting to bool from SQL BIGINT
    if used == 1 {
        return Err(ExternalInternalError {
            external_error: Some("This identifier is already used.".to_string()),
            internal_error: None,
        });
    }

    let player_id: i64;

    match discord_identifier_fetched_row.try_get("playerid") {
        Ok(good_id) => {
            player_id = good_id;
        }
        Err(error) => {
            return Err(ExternalInternalError {
                external_error: Some("Query error. Contact admin.".to_string()),
                internal_error: Some(
                    format!("Link: Error getting playerid from discord_identifiers row: {error:?}")
                        .to_string(),
                ),
            });
        }
    }

    let discord_link_execute_result =
        sqlx::query("INSERT INTO discord_links (player_id, discord_id) VALUES (?, ?)")
            .bind(player_id)
            .bind(discord_id.clone())
            .execute(&database_connection)
            .await;

    match discord_link_execute_result {
        Ok(_) => {}
        Err(error) => {
            return Err(ExternalInternalError {
                external_error: Some("Query error. Contact admin.".to_string()),
                internal_error: Some(
                    format!(
                    "Link: Error inserting player_id and discord_id into discord_links: {error:?}"
                )
                    .to_string(),
                ),
            });
        }
    }

    let used_update_execute_result =
        sqlx::query("UPDATE discord_identifiers SET used = 1 WHERE identifier = ?")
            .bind(identifier)
            .execute(&database_connection)
            .await;

    match used_update_execute_result {
        Ok(_) => {}
        Err(error) => {
            return Err(ExternalInternalError {
                external_error: Some("Query error. Contact admin.".to_string()),
                internal_error: Some(
                    format!("Link: Error updating used in discord_identifiers: {error:?}")
                        .to_string(),
                ),
            });
        }
    }

    let output = convert_to_safe_message(&format!("{} linked.", msg.author.name));
    if let Err(error) = msg.channel_id.say(&ctx.http, output).await {
        return Err(ExternalInternalError {
            external_error: Some("Link message failed?".to_string()),
            internal_error: Some(format!("Link: Error sending message: {error:?}").to_string()),
        });
    }

    return Ok(());
}
