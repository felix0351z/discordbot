use std::string::String;
use std::env;
use std::fmt::Display;
use log::info;

// Standard values
const CHAT_PREFIX: &str = "!";
const LAVALINK_HOST: &str = "127.0.0.1";
const LAVALINK_PORT: u32 = 2333;
const LAVALINK_PASSWORD: &str = "youshallnotpass";
const LAVALINK_SSL: bool = false;

pub struct Config {
    pub discord_token: String,
    pub chat_prefix: String,
    pub lavalink_host: String,
    pub lavalink_port: u32,
    pub lavalink_password: String,
    pub lavalink_ssl: bool
}

pub fn new() -> Config {
    let Ok(discord_token) = env::var("DISCORD_TOKEN") else {
        panic!("DISCORD_TOKEN environment variable is not set!");
    };
    let port = read_var("LAVALINK_PORT", LAVALINK_PORT).parse::<u32>().unwrap_or_else(|_| {
        panic!("LAVALINK_PORT environment variable is not a valid port number!");
    });
    let ssl = read_var("LAVALINK_SSL", LAVALINK_SSL).parse::<bool>().unwrap_or_else(|_| {
       panic!("LAVALINK_SSL environment variable is not a valid boolean value! Please use true or false to inform if lavalink uses ssl.");
    });

    Config {
        discord_token,
        chat_prefix: read_var("CHAT_PREFIX", CHAT_PREFIX),
        lavalink_host: read_var("LAVALINK_HOST", LAVALINK_HOST),
        lavalink_port: port,
        lavalink_password: read_var("LAVALINK_PASSWORD", LAVALINK_PASSWORD),
        lavalink_ssl: ssl,
    }
}

fn read_var<T: Display>(key: &str, default: T) -> String {
    env::var(key).unwrap_or_else(|_| {
        info!("{key} environment variable is not set. The standard {key} {default} will be used.");
        default.to_string()
    })
}