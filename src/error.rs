use log::error;
use poise::FrameworkError;
use crate::{Data, Error};
use crate::music::format::ErrorEmbedFormat;
use lavalink_rs::error::LavalinkError;

pub async fn error_handler(framework_err: FrameworkError<'_, Data, Error>) {
    match framework_err {
        FrameworkError::Setup { .. } => {}
        FrameworkError::Command { ref error, .. } => {
            // Check for specific error
            if let Some(lavalink_error) = error.downcast_ref::<LavalinkError>() {
                match lavalink_error {
                    LavalinkError::HyperClientError(..) => {
                        error!("No connection to Lavalink server!");
                    }
                    &_ => { error!("Lavalink: {}", error); }
                }
            } else {
                error!("Command execution: {}", error);
            }
        }
        _ => {}
    }
}

fn log_error<T: error::Error>(msg: &str, err: T) {
    error!("{msg}: {err}");
    let result = framework_err.ctx().unwrap().send(error.error_to_embed()).await;
    if let Err(fail) = result {
        error!("Failed to send error message to guild: {}", fail)
    }

}