use std::time::Duration;
use serenity::all::{ActivityData, Context, FullEvent, OnlineStatus};
use crate::{Data, Error};
use crate::music::inactivity_handler;
const LEAVE_TIME: u64 = 10*60; // 10 Minutes

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready {  .. } => {
            let activity = ActivityData::custom("!help für mehr Burgieees");
            ctx.shard.set_presence(Some(activity), OnlineStatus::Online);

            let songbird = songbird::get(ctx).await.unwrap();
            let lavalink = framework.user_data.lavalink.clone();
            let _task = inactivity_handler(Duration::from_secs(LEAVE_TIME), lavalink, songbird);
        },
        _ => {}
    }
    Ok(())
}

