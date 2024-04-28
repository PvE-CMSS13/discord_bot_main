mod command;

mod helper;

use crate::command::attempt_commands;
use crate::helper::convert_to_safe_message;

use dotenvy::dotenv;

use serenity::all::{Context, EventHandler, GatewayIntents, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;

use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};

use std::env;

struct DiscordHandler {
    database: Option<MySqlPool>,
}

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with("!") {
            return;
        }

        match ctx.http.application_id() {
            Some(our_id) => {
                if our_id.get() == msg.author.id.get() {
                    return;
                }
            }
            None => {
                println!("Unable to get our application id.");
                return;
            }
        }

        if let Err(error) = attempt_commands(&self, &ctx, &msg).await {
            if let Some(error_text) = error.internal_error {
                println!("{}", error_text);
            }

            if let Some(error_text) = error.external_error {
                let output = convert_to_safe_message(&error_text);
                if let Err(why) = msg.channel_id.say(&ctx.http, output).await {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => {}
        Err(_error) => {
            println!("No local .env found. This is normal if running in docker. Continuing.");
        }
    }

    let database = setup_sql_database().await;

    let token = env::var("DISCORD_TOKEN").expect("Expected token DISCORD_TOKEN in the environment");

    let discord_handler = DiscordHandler { database };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut discord_client = serenity::Client::builder(&token, intents)
        .event_handler(discord_handler)
        .await
        .expect("Err creating client.");

    discord_client.start().await.expect("Err starting client.");
}

//TODO: make this asynchronous, auto recovery
async fn setup_sql_database() -> Option<Pool<MySql>> {
    match env::var("DATABASE_URL") {
        Ok(unwrapped_url) => match MySqlPool::connect(&unwrapped_url).await {
            Ok(connection) => {
                println!("SQL Database connected.");
                Some(connection)
            }
            Err(error) => {
                println!("Unable to connect to database with URL. Disabling database functionality. Error: {error:?}");
                None
            }
        },
        Err(error) => {
            println!("No database URL found. Disabling database functionality. Error: {error:?}");
            None
        }
    }
}
