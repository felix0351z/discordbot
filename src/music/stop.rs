use std::time::Duration;
use crate::{Context, Error};
use crate::music::MusicCommandError::{NoTrackIsPlaying, NoUserInVoiceChannel};
use crate::music::PlayerStoppedExtension;

#[poise::command(prefix_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {

    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();

    let Some(player) = lavalink.get_player_context(guild_id) else {
        return Err(NoUserInVoiceChannel.into())
    };

    let now_playing = player.get_player().await?.track.is_some();

    if !now_playing {
        return Err(NoTrackIsPlaying.into());
    }

    player.stop_now().await?;
    player.mark_stop()?;
    ctx.say("Wiedegabe beendet!").await?;


    return Ok(())
}

/// Leave the current voice channel.
#[poise::command(slash_command, prefix_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let songbird = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let lava_client = ctx.data().lavalink.clone();

    lava_client.delete_player(guild_id).await?;
    if songbird.get(guild_id).is_some() {
        songbird.remove(guild_id).await?;
    }

    ctx.say("Left voice channel.").await?;

    Ok(())
}