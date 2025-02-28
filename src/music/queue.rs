use crate::{Context, Error};
use crate::format::EmbedFormat;
use crate::music::MusicCommandError::NoTrackIsPlaying;

/// Show the current queue
#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();

    let Some(player) = lavalink.get_player_context(guild_id) else {
        return Err(NoTrackIsPlaying.into())
    };
    if player.get_player().await?.track.is_none() {
        return Err(NoTrackIsPlaying.into())
    }

    let queue = player.get_queue().get_queue().await?;
    ctx.send(queue.as_embed_message("")).await?;

    Ok(())
}