use crate::{Context, Error};
use crate::music::MusicCommandError::NoTrackIsPlaying;
use crate::music::PlayerStoppedExtension;

/// Skip the current track
#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();

    let Some(player) = lavalink.get_player_context(guild_id) else {
        return Err(NoTrackIsPlaying.into());
    };
    if player.get_player().await?.track.is_none() {
        return Err(NoTrackIsPlaying.into());
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