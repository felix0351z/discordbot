use std::fs;
use std::path::Path;
use log::info;
use serde::{Deserialize, Serialize};

const CONFIG_NAME: &str = "settings.toml";

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub lavalink: LavalinkNodeSettings
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ApplicationSettings {
    pub discord_token: String,
    pub prefix: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LavalinkNodeSettings {
    pub hostname: String,
    pub port: i32,
    pub password: String,
    pub is_ssl: bool,
}

impl Settings {

    fn new() -> Self {
        Self {
            application: ApplicationSettings {
                discord_token: "".to_string(),
                prefix: "!".to_string(),
            },
            lavalink: LavalinkNodeSettings {
                hostname: "127.0.0.1".to_string(),
                port: 2333,
                password: "youshallnotpass".to_string(),
                is_ssl: false
            }
        }
    }


}


pub fn load_settings() -> Settings {
    let is_present = Path::new(CONFIG_NAME).exists();
    if !is_present {
        // Create standard configuration and write it
        let standard = toml::to_string(&Settings::new()).unwrap();
        fs::write(CONFIG_NAME, standard).unwrap_or_else(|err| {
            panic!("Failed to create a new config file: {err}", )
        });

        panic!("Successfully created configuration file {}. Please fill out the configuration and restart the application.", CONFIG_NAME)
    }

    let file = fs::read_to_string(CONFIG_NAME).unwrap_or_else(|err| {
        panic!("Failed to read config file {}: {}", CONFIG_NAME, err)
    });
    let config: Settings = toml::from_str(file.as_str()).unwrap_or_else(|_| {
        panic!("Failed to parse the config file. You may have an syntax error in your configuration!")
    });

    info!("Loaded configuration file {} successfully", CONFIG_NAME);
    return config;
}
