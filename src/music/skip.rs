use std::sync::Mutex;

use crate::{Context, Error};
use crate::music::MusicCommandError::{NoTrackInQueue, NoUserInVoiceChannel};
use crate::music::PlayerStoppedExtension;

#[poise::command(prefix_command, guild_only)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {

    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();

    let Some(player) = lavalink.get_player_context(guild_id) else {
        return Err(NoUserInVoiceChannel.into());
    };


    let is_playing = player.get_player().await?.track.is_some();
    if !is_playing {
        return Err(NoTrackInQueue.into());
    }

    // if there is no song left in queue, the player will stop
    let queue_size = player.get_queue().get_count().await?;
    if queue_size == 0 {
        // Remember that the player was stopped automatically
        player.mark_stop()?;
    }

    player.skip()?;
    ctx.say("Lied übersprungen!").await?;
    Ok(())
}