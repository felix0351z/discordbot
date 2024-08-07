use crate::{Context, Error};

#[poise::command(prefix_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {


    return Ok(())
}

/// Leave the current voice channel.
#[poise::command(slash_command, prefix_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {


    Ok(())
}