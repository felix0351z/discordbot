use poise::{Context, CreateReply, FrameworkError};
use crate::{Data, Error};
use lavalink_rs::error::LavalinkError;
use serenity::all::{Color, CreateEmbed};
use crate::music::MusicCommandError;

type PoiseContext<'a> = Option<Context<'a, Data, Error>>;

pub async fn error_handler(framework_err: FrameworkError<'_, Data, Error>) {
    // get the poise context to interact with the text channel later.
    let ctx = framework_err.ctx();

    match framework_err {
        FrameworkError::Setup { ref error, .. } => { log::error!("Initial setup: {}", error); }
        FrameworkError::Command { ref error, .. } => {
            // Handle all known errors
            if let Some(lavalink_error) = error.downcast_ref::<LavalinkError>() { handle_lavalink_error(ctx, lavalink_error).await; }
            else if let Some(music_error) = error.downcast_ref::<MusicCommandError>() { handle_music_error(ctx, music_error).await; }

            // If the error is not known yet, log it to the console
            else {
                log::error!("Command execution: {}", error);
            }
        }
        _ => {}
    }
}


/// Handle a thrown lavalink error
async fn handle_lavalink_error(ctx: PoiseContext<'_>, error: &LavalinkError) {
    match error {
        LavalinkError::HyperClientError(..) => {
            // Notify that no successfully connection was established yet
            log_error(ctx, "No connection to lavalink server!", error).await;
        }
        LavalinkError::TrackError(..) => {
            // Notify that lavalink has a problem with the current track
            log_error(ctx, "Lavalink:", error).await;
        }
        // If the error is not known yet, log it to the console
        &_ => { log::error!("Lavalink: {}", error); }
    }
}

/// Handle a thrown music error
async fn handle_music_error(ctx: PoiseContext<'_>, error: &MusicCommandError) {
    // Don't log the error in the console, because it is a user error only
    if let Some(ctx) = ctx {
        let _ = ctx.send(create_embed(error.to_string())).await;
    }
}


/// Log the error in the console and notify the user in the guild channel, if a request or was made.
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

/// Create a CreateReply object from a text string
fn create_embed(msg: impl Into<String>) -> CreateReply {
    let creator = CreateEmbed::new()
        .description(msg)
        .color(Color::RED);

    CreateReply::default().embed(creator)
}