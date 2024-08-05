use std::collections::HashMap;
use std::fmt::Pointer;
use std::panic::RefUnwindSafe;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use futures::future::err;
use lavalink_rs::client::LavalinkClient;
use lavalink_rs::error::LavalinkResult;
use lavalink_rs::hook;
use lavalink_rs::model::events::{PlayerUpdate, TrackEnd};
use lavalink_rs::model::GuildId;
use lavalink_rs::player_context::PlayerContext;
use log::{error, info, warn};
use serenity::all::{ChannelId, VoiceState};
use songbird::Songbird;
use thiserror::Error;
use tokio::task::JoinHandle;
use crate::{Context, Error};

pub mod play;
mod format;
pub mod skip;
pub mod stop;
pub mod info;
pub mod queue;

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &lavalink_rs::model::events::Ready) {
    // if the bot started, remove all existent player and give status info
    client.delete_all_player_contexts().await.unwrap();
    info!("{:?} -> {:?}", session_id, event);
}




#[derive(Error, Debug)]
pub enum MusicCommandError {
    #[error("The channel available. To play musics the user has to be in a voice channel")]
    NoUserInVoiceChannel,
    #[error("You need to add what I should play")]
    NoQueryProvided,
    #[error("Can't load the track")]
    FailedLoadingTrack,
    #[error("No song in queue")]
    NoTrackInQueue,
    #[error("No track is playing")]
    NoTrackIsPlaying,
    #[error("Failed to get an player context")]
    NoPlayerContext
}


pub trait PlayerStoppedExtension {

    fn is_stopped(&self) -> Result<bool, Error>;
    fn mark_stop(&self) -> Result<(), Error>;

}

impl PlayerStoppedExtension for PlayerContext {
    fn is_stopped(&self) -> Result<bool, Error> {
        let mutex = self.data::<Mutex<bool>>()?;
        let mut value = mutex.lock().unwrap();
        if *value {
            *value = false;
            return Ok(true)
        }

        Ok(false)
    }

    fn mark_stop(&self) -> Result<(), Error> {
        let mutex = self.data::<Mutex<bool>>()?;
        let mut value = mutex.lock().unwrap();
        *value = true;

        Ok(())
    }
}




