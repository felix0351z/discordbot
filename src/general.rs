use poise::builtins::HelpConfiguration;
use crate::{Context, Error};

/// Ping Command, say hello to your favourite bot
#[poise::command(prefix_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {

    let name = &ctx.author().name;
    ctx.say(format!("Hey {} I'm BigMac", name)).await?;

    Ok(())
}

/// Pong!
#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {

    let latency = ctx.ping().await.as_millis();
    ctx.say(format!("Pong! The latency is {}ms", latency)).await?;

    Ok(())
}

/// Get help for commands
#[poise::command(prefix_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {

    let config = HelpConfiguration {
        extra_text_at_bottom: "Type !help command for more info on a command.",
        ephemeral: true,
        show_context_menu_commands: false,
        show_subcommands: false,
        include_description: true,
        __non_exhaustive: (),
    };
    poise::builtins::help(ctx, None, config).await?;
    Ok(())
}