use std::fmt::Display;

use chrono::{self, Utc};
use metamodel::{now, AggregateRoot, Entity};

pub struct Album {
    // Album data
    title: String,
    // artist: String,
    // year: i32,
    // genre: String,
    // tracks: Vec<Track>,
}

impl Display for Album {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Album: {}", self.title)
    }
}

// impl Album {
//     pub fn update_title(&mut self, title: String) {
//         let event = now(Event::TitleUpdated {
//             title,
//         });

//         self.apply_event(event);
//         // publish(event);
//     }
// }

pub enum Event {
    Created {
        // album data
        title: String,
    },
    TitleUpdated {

        // album data
        title: String,
    }
}

impl AggregateRoot for Album {
    type Event = Event;

    fn create_impl(event: &Self::Event) -> Self {
        match event {
            Event::Created { title } => Album {
                title: title.clone(),
            },
            _ => panic!("Event not supported")
        }
    }

    fn apply_event_impl(album: &mut Album, event: &Event) {
        match event {
            Event::TitleUpdated { title } =>  {album.title = title.clone()}
            _ => panic!("Event not supported")
        }
    }
} 

pub fn add_album(title: String) -> Entity<Album>{
    let event = now(Event::Created {
        title,
    });

    let album = Album::create(event);
    // publish(event);
    album
}
