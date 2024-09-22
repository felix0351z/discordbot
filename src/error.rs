use log::error;
use poise::FrameworkError;
use crate::{Data, Error};
use crate::music::format::ErrorEmbedFormat;

pub async fn error_handler(framework_err: FrameworkError<'_, Data, Error>) {
    match framework_err {
        FrameworkError::Setup { .. } => {}
        FrameworkError::Command { ref error, .. } => {
            let result = framework_err.ctx().unwrap().send(error.error_to_embed()).await;
            if let Err(fail) = result {
                error!("Failed to send error message to guild: {}", fail)
            }

            error!("Error occurred while command execution: {}", error.to_string());
        }
        _ => {}
    }


}