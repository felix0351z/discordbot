use std::sync::Mutex;
use lavalink_rs::client::LavalinkClient;
use lavalink_rs::hook;
use lavalink_rs::player_context::PlayerContext;
use log::{error, info};
use thiserror::Error;
use crate::Error;

// Commands
pub mod play; pub mod skip; pub mod stop; pub mod info; pub mod queue; pub mod clear;
pub mod leave; pub mod lavalink;

// Formatting extensions
pub mod format;


#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &lavalink_rs::model::events::Ready) {
    // if the bot started, remove all existent players and give status info
    client.delete_all_player_contexts().await.unwrap();
    info!("{:?} -> {:?}", session_id, event);
}

/// General music error, which will be thrown/send if the interaction between the user and the bot is not correct
#[derive(Error, Debug)]
pub enum MusicCommandError {
    #[error("You have to be in a voice channel to play music!")]
    NoUserInVoiceChannel,
    #[error("What do you want to play?")]
    NoQueryProvided,
    #[error("Can't load the given track")]
    FailedLoadingTrack,
    #[error("No track is playing")]
    NoTrackIsPlaying,
}


/// Due to an issue with the lavalink crate, the player must be manually skipped, if the player was stopped before.
/// To control that a boolean can be used, to remember if the player was stopped
pub trait PlayerStoppedExtension {

    fn is_stopped(&self) -> Result<bool, Error>;
    fn mark_stop(&self) -> Result<(), Error>;

}

impl PlayerStoppedExtension for PlayerContext {

    /// Check if the player was stopped manually
    fn is_stopped(&self) -> Result<bool, Error> {
        let mutex = self.data::<Mutex<bool>>()?;
        let mut value = mutex.lock().unwrap();
        if *value {
            *value = false;
            return Ok(true)
        }

        Ok(false)
    }

    /// Mark that the player was stopped
    fn mark_stop(&self) -> Result<(), Error> {
        let mutex = self.data::<Mutex<bool>>()?;
        let mut value = mutex.lock().unwrap();
        *value = true;

        Ok(())
    }
}




