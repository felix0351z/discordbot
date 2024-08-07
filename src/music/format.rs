
use poise::CreateReply;
use serenity::all::{Color, CreateEmbed, CreateEmbedAuthor};
use songbird::input::AuxMetadata;

pub trait EmbedFormat {
    fn as_embed_message(&self, prefix: &str) -> CreateReply;
}

impl EmbedFormat for AuxMetadata {
    fn as_embed_message(&self, prefix: &str) -> CreateReply {
        let author = self.artist.as_deref().unwrap_or_default();
        let title = self.title.as_deref().unwrap_or_default();

        let mut creator = CreateEmbed::new()
            .author(CreateEmbedAuthor::new(author))
            .title(format!("{}:\n{:?}", prefix, title))
            .color(Color::RED);


        if let Some(time) = self.duration {
            let minutes = (time.as_secs()) / 60;
            let seconds = (time.as_secs()) % 60;
            creator = creator.description(format!("{}:{:02}", minutes, seconds));
        }
        if let Some(uri) = self.source_url.as_deref() {
            creator = creator.url(uri);
        }
        if let Some(thumbnail) = self.thumbnail.as_deref() {
            creator = creator.image(thumbnail);
        }

        return CreateReply::default().embed(creator)
    }
}

/*impl EmbedFormat for PlaylistInfo {
    fn as_embed_message(&self, prefix: &str) -> CreateReply {
        let creator = CreateEmbed::new()
            .title(format!("{}:\nPlaylist - {}", prefix,  self.name))
            .color(Color::RED);

        return CreateReply::default().embed(creator)
    }
}*/

/*impl EmbedFormat for VecDeque<TrackInQueue> {
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

}*/