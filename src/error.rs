use log::error;
use poise::FrameworkError;
use crate::{Data, Error};
use crate::music::format::EmbedFormat;
use crate::music::MusicCommandError;

pub async fn error_handler(framework_err: FrameworkError<'_, Data, Error>) {
    match framework_err {
        FrameworkError::Setup { .. } => {}
        FrameworkError::Command { ref error, .. } => {
            if let Some(music_error) = error.downcast_ref::<MusicCommandError>() {
                // if it is a common error, write to the user
                let err = framework_err.ctx().unwrap().send(music_error.as_embed_message("")).await;
                if let Err(fail) = err { error!("Failed to send message to guild: {}", fail) }
            } else {
                // if unknown log the error
                error!("Unknown error happened while command execution: {}", error.to_string())
            }
        }
        _ => {}
    }


}