use std::ops::Deref;
use std::sync::{Arc, Mutex};
use lavalink_rs::player_context::{PlayerContext, TrackInQueue};
use lavalink_rs::prelude::{SearchEngines, TrackLoadData};
use serenity::all::GuildId;

use crate::{Context, Error};
use crate::music::{MusicCommandError, PlayerStoppedExtension};
use crate::music::MusicCommandError::{FailedLoadingTrack, NoQueryProvided, NoUserInVoiceChannel};
use crate::music::format::EmbedFormat;

async fn join(
    ctx: &Context<'_>,
    guild_id: GuildId,
) -> Result<(PlayerContext), Error>{
    let lavalink = ctx.data().lavalink.clone();
    let songbird = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    // Check if a player exists in this guild at that moment
    let player_context = lavalink.get_player_context(guild_id);

    if player_context.is_some() {
        return Ok(player_context.unwrap())
    }

    // If not, join a channel and create the context
    // If the user isn't in a voice channel, return error
    let guild = ctx.guild().unwrap().deref().clone();
    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or::<MusicCommandError>(NoUserInVoiceChannel.into())
        ?;
    let (connection_info, call) = songbird.join_gateway(guild_id, channel_id).await?;


    // If the bot joins the channel successfully, create the player context
    let created_player = lavalink.create_player_context_with_data::<Mutex<bool>>(
        guild_id,
        connection_info,
        // Give him an additional boolean, to see if the player is stopped
        Arc::from(Mutex::new(false))
    ).await?;

    // If a new player context was created => return true
    Ok(created_player)
}

#[poise::command(prefix_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search or URL"]
    #[rest]
    search: Option<String>
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();

    // If no input provided, return error
    let Some(search) = search else {
        return Err(NoQueryProvided.into());
    };

    // Search for the query
    let query = match search.starts_with("http") {
        true => search,
        false => {
            SearchEngines::YouTube.to_query(search.as_str())?
        }
    };

    // Load tracks
    let loaded_track = lavalink.load_tracks(guild_id, query.as_str()).await?;
    let mut playlist_info = None;

    let mut tracks: Vec<TrackInQueue> = match loaded_track.data {
        Some(TrackLoadData::Track(v)) => {
            // take the video
            vec![v.into()]
        }
        Some(TrackLoadData::Playlist(v)) => {
            // Take the complete playlist
            playlist_info = Some(v.info);
            v.tracks
                .iter()
                .map(|track| track.clone().into())
                .collect()
        }
        Some(TrackLoadData::Search(v)) => {
            // take the first search result
            vec![v[0].clone().into()]
        }
        _ => {
            return Err(FailedLoadingTrack.into())
        }
    };

    // Create the player
    let player = join(&ctx, guild_id).await?;

    // Reply track info:
    let is_playing = player.get_player().await?.track.is_some();
    let text = if is_playing { "Zur Warteschlange hinzugefügt" } else { "Spiele jetzt" };

    match playlist_info {
        None => {
            let track = &tracks[0].track;
            ctx.send(track.as_embed_message(text)).await?;
        }
        Some(info) => {
            ctx.send(info.as_embed_message(text)).await?;
        }
    }

    // Add the track/playlist to the queue
    // Note: The lavalink-rs will start the playback automatically
    let queue = player.get_queue();
    queue.append(tracks.into())?;

    if player.is_stopped()? { player.skip()?; }
    Ok(())
}

