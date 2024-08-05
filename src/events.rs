use serenity::all::{ActivityData, Context, FullEvent, OnlineStatus};

use crate::{Data, Error};

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("{} started and is ready!", data_about_bot.user.name);

            let activity = ActivityData::custom("!help für mehr Burgieees");
            ctx.shard.set_presence(Some(activity), OnlineStatus::Online);
        },

        _ => {}
    }
    Ok(())
}

