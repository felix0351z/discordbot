use std::ops::Deref;
use std::sync::{Arc, Mutex};
use lavalink_rs::player_context::{PlayerContext, TrackInQueue};
use lavalink_rs::prelude::{SearchEngines, TrackLoadData};
use serenity::all::GuildId;

use crate::{Context, Error};
use crate::music::{MusicCommandError, PlayerStoppedExtension};
use crate::music::MusicCommandError::{FailedLoadingTrack, NoQueryProvided, NoUserInVoiceChannel};
use crate::format::EmbedFormat;

async fn join(
    ctx: &Context<'_>,
    guild_id: GuildId,
) -> Result<PlayerContext, Error>{
    let lavalink = ctx.data().lavalink.clone();
    let songbird = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    
    // If the user isn't in a voice channel, return error
    let guild = ctx.guild().unwrap().deref().clone();
    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or::<MusicCommandError>(NoUserInVoiceChannel.into())
        ?;

    // Check if there is already an active listening session
    if let Some(call) = songbird.get(guild_id) {
        let handler = call.lock().await;

        // If yes and the user remains in the same channel, the player can be returned as it is
        if handler.current_channel().is_some_and(|it| it.0.get() == channel_id.get()) {
            if let Some(player) = lavalink.get_player_context(guild_id) {
                return Ok(player)
            }

        } else {
            // If not, the old player must be deleted and a new player has to be created in the new channel
            lavalink.delete_player(guild_id).await?;
        }
    }

    // If a new player must be created, join/switch the channel and create the context
    let (connection_info, _call) = songbird.join_gateway(guild_id, channel_id).await?;

    // If the bot joins the channel successfully, create the player context
    let created_player = lavalink.create_player_context_with_data::<Mutex<bool>>(
        guild_id,
        connection_info,
        // Give him an additional boolean, to see if the player is stopped
        Arc::from(Mutex::new(false))
    ).await?;
    Ok(created_player)
}

/// Play music from YouTube via url or search request
#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search or URL"]
    #[rest]
    search: Option<String>
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();

    // If no search query provided, return
    let Some(search) = search else {
        return Err(NoQueryProvided.into());
    };

    // Search for the query if no link was provided
    let query = match search.starts_with("http") {
        true => search,
        false => { SearchEngines::YouTube.to_query(search.as_str())? }
    };

    // Load tracks
    let loaded_track = lavalink.load_tracks(guild_id, query.as_str()).await?;
    let mut playlist_info = None;
    let tracks: Vec<TrackInQueue> = match loaded_track.data {
        Some(TrackLoadData::Track(v)) => {
            // If a video was given, wrap the track around a vec
            vec![v.into()]
        }
        Some(TrackLoadData::Playlist(v)) => {
            // If a playlist was given, map the playlist
            playlist_info = Some(v.info);
            v.tracks
                .iter()
                .map(|track| track.clone().into())
                .collect()
        }
        Some(TrackLoadData::Search(v)) => {
            // If a search was given, only take the first result
            vec![v[0].clone().into()]
        }
        _ => {
            return Err(FailedLoadingTrack.into())
        }
    };

    // Create the voice session
    let player = join(&ctx, guild_id).await?;

    // Send information what data will be played
    let is_playing = player.get_player().await?.track.is_some();
    let text = if is_playing { "Zur Warteschlange hinzugefügt" } else { "Spiele jetzt" };

    if let Some(info) = playlist_info {
        ctx.send(info.as_embed_message(text)).await?;
    } else {
        let track = &tracks[0].track;
        ctx.send(track.as_embed_message(text)).await?;
    }

    // Add the track/playlist to the queue
    // Note: The lavalink-rs will start the playback automatically, if it wasn't manually stopped
    let queue = player.get_queue();
    queue.append(tracks.into())?;

    if player.is_stopped()? { player.skip()?; }
    Ok(())
}

