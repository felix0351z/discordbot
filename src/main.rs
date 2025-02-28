use std::default::Default;
use std::env;
use std::sync::Arc;

use lavalink_rs::client::LavalinkClient;
use lavalink_rs::model::events::Events;
use lavalink_rs::node::NodeBuilder;
use lavalink_rs::prelude::NodeDistributionStrategy;
use log::info;
use poise::{Framework, FrameworkError, FrameworkOptions, PrefixFrameworkOptions};
use serenity::all::GatewayIntents;
use serenity::Client;
use songbird::SerenityInit;
use crate::music::info::info;

// General command
mod commands;

// Event handler
mod events;

// Error handler
mod error;

// All music related code
mod music;

// Config manager
mod config;

// Custom user data passed to all command functions
pub struct Data {
    lavalink: Arc<LavalinkClient>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


#[tokio::main]
async fn main() {
    // Start logging
    if env::var("RUST_LOG").is_err() { env::set_var("RUST_LOG", "DiscordBot=info") }
    env_logger::init();

    // Load configuration
    let settings = config::new();
    info!("Loaded configuration.");

    //Initialise poise framework for command management
    let options = FrameworkOptions {
        commands: vec![
            commands::hello(), commands::ping(), commands::help(),
            music::play::play(), music::skip::skip(), music::stop::stop(),
            music::info::info(), music::queue::queue(), music::clear::clear(),
            music::leave::leave(), music::lavalink::lavalink(),
        ],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(settings.chat_prefix),
            mention_as_prefix: true,
            ..Default::default()
        },
        // error handler for all errors which occur
        on_error: |framework_err: FrameworkError<'_, Data, Error>| {
            Box::pin(error::error_handler(framework_err))
        },
        event_handler: |ctx, event, framework, _data| {
            Box::pin(events::event_handler(ctx, event, framework))
        },
        ..Default::default()
    };

    let poise_framework = Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                // Register  commands of the bot at the discord server
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                // Load lavalink
                let events = Events {
                    ready: Some(music::ready_event),
                    ..Default::default()
                };
                let node = NodeBuilder {
                        hostname: format!("{}:{}", settings.lavalink_host, settings.lavalink_port).to_string(),
                        password: settings.lavalink_password,
                        is_ssl: settings.lavalink_ssl,
                        events: Events::default(),
                        user_id: ctx.cache.current_user().id.into(),
                        session_id: None
                };
                let client = LavalinkClient::new(events, vec![node], NodeDistributionStrategy::round_robin()).await;

                Ok(Data {
                    lavalink: Arc::new(client)
                })
            })
        })
        .options(options)
        .build();
    info!("Poise framework initialized.");

    // Create the serenity client and start the server
    let mut client = Client::builder(settings.discord_token, GatewayIntents::all())
        .register_songbird()
        .framework(poise_framework)
        .await
        .expect("Error creating client");
    info!("Client created.");

    // Start the client
    client.start().await
        .expect("Error client runtime");
}





