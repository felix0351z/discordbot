use crate::{Context, Error};
use crate::music::MusicCommandError::NoTrackIsPlaying;

/// Clear the queue
#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();

    let Some(player) = lavalink.get_player_context(guild_id) else {
        return Err(NoTrackIsPlaying.into())
    };
    if player.get_player().await?.track.is_none() {
        return Err(NoTrackIsPlaying.into())
    }

    let queue = player.get_queue();
    queue.clear()?;

    ctx.say("Warteschlange wurde gelöscht.").await?;
    Ok(())
}