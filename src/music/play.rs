use std::fs::metadata;
use std::ops::Deref;

use poise::async_trait;
use serenity::all::GuildId;
use songbird::{Event, EventContext, TrackEvent};
use songbird::input::{Input, YoutubeDl};

use crate::{Context, Error};
use crate::music::MusicCommandError;
use crate::music::MusicCommandError::{NoQueryProvided, NoUserInVoiceChannel};

use reqwest::Client;

struct EventErrorNotifier;

#[async_trait]
impl songbird::EventHandler for EventErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!("Track {:?} encountered an error: {:?}", handle.uuid(), state.playing
                );
            }
        }

        None
    }
}

async fn join(
    ctx: &Context<'_>,
    guild_id: GuildId,
) -> Result<(), Error>{
    // Get the songbird manager
    let songbird = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    // If not, join a channel and create the context
    // If the user isn't in a voice channel, return error
    let guild = ctx.guild().unwrap().deref().clone();
    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or::<MusicCommandError>(NoUserInVoiceChannel.into())
        ?;

    if let Ok(handler_lock) = songbird.join(guild_id, channel_id).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), EventErrorNotifier)
    }

    Ok(())
}

#[poise::command(prefix_command, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search or URL"]
    #[rest]
    search: Option<String>
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let songbird = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let http_client: Client =  ctx.data().http_client.clone();

    // If no input provided, return error
    let Some(search) = search else {
        return Err(NoQueryProvided.into());
    };

    join(&ctx, guild_id).await?;

    if let Some(handler_lock) = songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let src = if search.starts_with("http") {
            YoutubeDl::new(http_client, search)
        } else {
            YoutubeDl::new_search(http_client, search)
        };

        let mut input: Input = src.clone().into();
        let meta = input.aux_metadata().await?;

        println!("{:?} -  {:?}", meta.artist, meta.title);


        let _ = handler.play_input(input);
    }

    Ok(())
}

