use crate::{Context, Error};

#[poise::command(prefix_command, guild_only)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {


    Ok(())
}

#[poise::command(prefix_command, guild_only)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {

    Ok(())
}