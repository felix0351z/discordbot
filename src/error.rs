use poise::{Context, CreateReply, FrameworkError};
use crate::{Data, Error};
use lavalink_rs::error::LavalinkError;
use serenity::all::{Color, CreateEmbed};
use crate::music::MusicCommandError;

type PoiseContext<'a> = Option<Context<'a, Data, Error>>;

pub async fn error_handler(framework_err: FrameworkError<'_, Data, Error>) {
    let ctx = framework_err.ctx();

    match framework_err {
        FrameworkError::Setup { .. } => {}
        FrameworkError::Command { ref error, .. } => {
            // Check for specific error
            if let Some(lavalink_error) = error.downcast_ref::<LavalinkError>() {
                match lavalink_error {
                    LavalinkError::HyperClientError(..) => {
                        log_error(ctx, "No connection to lavalink server!", lavalink_error).await;
                    }
                    &_ => { log::error!("Lavalink: {}", error); }
                }
            }
            else if let Some(music_error) = error.downcast_ref::<MusicCommandError>() {
                // Send message to channel if a music error occurs
                // No log in console because it is no real error
                if let Some(ctx) = ctx {
                    let _ = ctx.send(create_embed(music_error.to_string())).await;
                }
            }
            else {
                log::error!("Command execution: {}", error);
            }
        }
        _ => {}
    }
}


async fn log_error<T: std::error::Error>(ctx: PoiseContext<'_>, msg: &str, err: T) {
    // Log message in console
    log::error!("{msg}: {err}");

    // Log message in channel
    if let Some(ctx) = ctx {
        let response = ctx.send(create_embed(msg)).await;

        // Log failure in channel
        if let Err(fail) = response {
            log::error!("Failed to send error message to guild: {}", fail)
        }
    }
}

fn create_embed(msg: impl Into<String>) -> CreateReply {
    let creator = CreateEmbed::new()
        .description(msg)
        .color(Color::RED);

    CreateReply::default().embed(creator)
}