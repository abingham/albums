use std::fmt::Display;

use aggregate_root::AggregateRoot;
use metamodel::entity::{Entity, EntityAttrs, InstanceId, UniqueId};
use metamodel::event::now;
use metamodel::aggregate_root::AggregateRoot;

#[derive(AggregateRoot)]
pub struct Album {
    // TODO: Can we use a macro to generate this boilerplate?
    entity_attrs: EntityAttrs,

    // Album data
    pub title: String,
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
    },
}

impl AggregateRoot for Album {
    type Event = Event;

    fn create_impl(event: &Self::Event) -> Self {
        match event {
            Event::Created { title } => Album {
                entity_attrs: EntityAttrs::new(),
                title: title.clone(),
            },
            _ => panic!("Event not supported"),
        }
    }

    fn apply_event_impl(&mut self, event: &Event) {
        match event {
            Event::TitleUpdated { title } => self.title = title.clone(),
            _ => panic!("Event not supported"),
        }
    }
}

pub fn add_album(title: String) -> Album {
    let event = now(Event::Created { title });

    Album::create(event)
}
