use std::fmt::Pointer;
use std::panic::RefUnwindSafe;

use log::error;
use thiserror::Error;

pub mod play;
mod format;
pub mod skip;
pub mod stop;
pub mod info;
pub mod queue;

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






