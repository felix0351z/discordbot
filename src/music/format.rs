use std::collections::VecDeque;
use lavalink_rs::model::track::{PlaylistInfo, TrackData};
use lavalink_rs::prelude::TrackInQueue;
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed, CreateEmbedAuthor};

pub trait EmbedFormat {
    fn as_embed_message(&self, prefix: &str) -> CreateReply;
}

impl EmbedFormat for TrackData {
    fn as_embed_message(&self, prefix: &str) -> CreateReply {
        let minutes = (self.info.length/1000) / 60;
        let seconds = (self.info.length/1000) % 60;

        let mut creator = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(&self.info.author))
            .title(format!("{}:\n{}", prefix, self.info.title))
            .color(Color::RED)
            .description(format!("{}:{:02}", minutes, seconds));

        match &self.info.uri {
            None => {}
            Some(it) => {
                creator = creator.url(it).image(it);
            }
        }

        match &self.info.artwork_url {
            None => {}
            Some(it) => {
                creator = creator.image(it);
            }
        }


        return CreateReply::default().embed(creator)
    }
}

impl EmbedFormat for PlaylistInfo {
    fn as_embed_message(&self, prefix: &str) -> CreateReply {
        let creator = CreateEmbed::new()
            .title(format!("{}:\nPlaylist - {}", prefix,  self.name))
            .color(Color::RED);

        return CreateReply::default().embed(creator)
    }
}

impl EmbedFormat for VecDeque<TrackInQueue> {
    fn as_embed_message(&self, prefix: &str) -> CreateReply {
        // If queue is empty return
        if self.is_empty() {
            return CreateReply::default().content("Die Warteschlange ist leer!");
        }
        let mut creator = CreateEmbed::new().title("Aktuelle Warteschlange:");
        let mut text = "".to_string();

        for (i, track) in self.iter().enumerate() {
            let time = &track.track.info.length;
            let minutes = (time/1000) / 60;
            let seconds = (time/1000) % 60;

            text.push_str(format!("{}: {} | {}:{:02}\n", i+1, track.track.info.title, minutes, seconds).as_str());
        }
        creator = creator.description(text);


        return CreateReply::default().embed(creator)
    }

}