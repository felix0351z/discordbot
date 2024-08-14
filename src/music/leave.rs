use crate::{Context, Error};
use crate::music::MusicCommandError::NoTrackIsPlaying;

/// Stop the playback and leave the voice channel
#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let songbird = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let lavalink = ctx.data().lavalink.clone();

    if lavalink.get_player_context(guild_id).is_none() {
        return Err(NoTrackIsPlaying.into())
    }

    lavalink.delete_player(guild_id).await?;
    if songbird.get(guild_id).is_some() {
        songbird.remove(guild_id).await?;
    }

    ctx.say("Left voice channel.").await?;
    Ok(())
}