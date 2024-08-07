use crate::{Context, Error};

#[poise::command(prefix_command, guild_only)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {


    Ok(())
}