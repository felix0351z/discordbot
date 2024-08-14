use poise::CreateReply;
use serenity::all::{Color, CreateEmbed};

use crate::{Context, Error};

/// Show the current lavalink statistics
#[poise::command(prefix_command, slash_command, guild_only, category = "Music")]
pub async fn lavalink(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let lavalink = ctx.data().lavalink.clone();



    let info = lavalink.request_info(guild_id).await?;
    let stats = lavalink.request_stats(guild_id).await?;

    let embed = CreateEmbed::new()
        .title(format!("Lavalink Server v.{} - JVM {}" , info.lavaplayer, info.jvm))
        .color(Color::RED)
        .url("https://lavalink.dev/")
        .field("Active players", format!("{}/{} Player", stats.playing_players, stats.players), false)
        .field("CPU usage", format!("{} Cores - {:.4}%", stats.cpu.cores, stats.cpu.system_load), false)
        .field("Lavalink load", format!("{:.4}%", stats.cpu.lavalink_load), false);

    let reply = CreateReply::default().embed(embed);
    ctx.send(reply).await?;

    Ok(())
}