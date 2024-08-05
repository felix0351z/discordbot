use crate::{Context, Error};
use crate::music::format::EmbedFormat;
use crate::music::MusicCommandError::NoTrackIsPlaying;

#[poise::command(prefix_command, guild_only)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {


    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();


    let Some(player) = lavalink.get_player_context(guild_id) else {
        return Err(NoTrackIsPlaying.into());
    };

    let Some(current_track) = player.get_player().await?.track else {
        return Err(NoTrackIsPlaying.into());
    };

    ctx.send(current_track.as_embed_message("Spiele gerade")).await?;

    Ok(())
}