use std::default::Default;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use futures::stream::IntoAsyncRead;
use log::{debug, error, info, warn};
use poise::{Framework, FrameworkError, FrameworkOptions, Prefix, PrefixFrameworkOptions};
use serenity::all::{EventHandler, GatewayIntents, GuildId, Shard, ShardManager};
use serenity::Client;
use serenity::prelude::TypeMapKey;
use songbird::SerenityInit;
use tokio::task::JoinHandle;
//TODO: Error handling
//TODO: Comments
//TODO: Command group + help

use reqwest::Client as HttpClient;


mod general;
mod events;
mod music;

const DISCORD_TOKEN: &str = "NDEyOTcyMzAxNDg3NzAyMDE2.GPFh0K.CFHb0w9ZKGk0EsiEMd52xM5zbU1B39hfQgMTC0";


type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    http_client: HttpClient
}

pub struct SerenityData {

}


#[tokio::main]
async fn main() {
    // Start the logger
    env_logger::init();



    // Provide intents, which are needed for this bot
    let intents =
        GatewayIntents::all();

    //Initialise the poise framework for command management
    let options = FrameworkOptions {
        commands: vec![
            general::hello(),
            general::ping(),
            general::help(),
            music::play::play(),
            music::skip::skip(),
            music::stop::stop(),
            music::info::info(),
            music::queue::queue(),
            music::queue::clear(),
            music::stop::leave(),
        ]
        ,
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("!".into()),
            additional_prefixes: vec![Prefix::Literal("Hey Bigmac,")],
            ..Default::default()
        },
        //Global error handler for all errors which occur
        on_error: |error: FrameworkError<'_, Data, Error>| Box::pin(async move {

            error!("{}" ,error)


        }),
        // Runs before every command
        pre_command: |ctx| Box::pin(async move{

        }),
        //Runs after every command
        post_command: |ctx| Box::pin(async move{

        }),
        event_handler: |ctx, event, framework, data| {
            Box::pin(events::event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    let poise_framework = Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", ready.user.name);
                // Register the commands of the bot at the discord server
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                // Load lavalink

                Ok(Data {
                    http_client: HttpClient::new()
                })
            })
        })
        .options(options)
        .build();




    // Create the client
    let mut client = Client::builder(DISCORD_TOKEN, intents)
        .register_songbird()
        .framework(poise_framework)
        .await
        .expect("Error while creating client");

    // Start the client
    if let Err(error) = client.start().await {
        println!("Error while client runtime: {error:?}")
    }
}





