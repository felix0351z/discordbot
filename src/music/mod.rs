use std::sync::{Arc, Mutex};
use std::time::Duration;
use lavalink_rs::client::LavalinkClient;
use lavalink_rs::hook;
use lavalink_rs::player_context::PlayerContext;
use log::{error, info};
use serenity::all::GuildId;
use songbird::Songbird;
use thiserror::Error;
use tokio::task::JoinHandle;
use crate::Error;

// Commands
pub mod play; pub mod skip; pub mod stop; pub mod info; pub mod queue; pub mod clear;
pub mod leave; pub mod lavalink;

// Formatting extensions
pub mod format;

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, _event: &lavalink_rs::model::events::Ready) {
    // Remove all existent players and give status info
    client.delete_all_player_contexts().await.unwrap();
    info!("Lavalink client ready. Session ID is {session_id}");
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


/// Asynchronously checks if a player is not playing anymore. If the case is true, the player will be closed due to inactivity
pub fn inactivity_handler(
    delay: Duration,
    lavalink: Arc<LavalinkClient>,
    songbird: Arc<Songbird>
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(delay).await;

            let guild_ids = lavalink.players.iter()
                .filter_map(|i|i.0.load().clone().map(|x|GuildId::new(x.guild_id.0)))
                .collect::<Vec<_>>();


            for guild_id in guild_ids.iter() {
                // If the player is active skip the current player
                if let Some(player) = lavalink.get_player_context(*guild_id) {
                    if player.get_player().await.is_ok_and(|it|it.track.is_some()) {
                        info!("Skipped player close for player at guild {} because he is playing", guild_id.get());
                        continue;
                    }
                }

                if lavalink.delete_player(*guild_id).await.is_ok() {
                    if songbird.get(*guild_id).is_some() {
                        let leave_request = songbird.remove(*guild_id).await;
                        match leave_request {
                            Ok(_) => {
                                info!("Leaved voice channel at guild {} because of inactivity", guild_id.get());
                            }
                            Err(err) => {
                                error!("Failed to leave voice channel at guild {}: {}", guild_id.get(), err)
                            }
                        }
                    }
                } else {
                    error!("Failed to leave voice channel at guild {}: Can't get the player context", guild_id.get())
                }
            }
        }
    })
}




