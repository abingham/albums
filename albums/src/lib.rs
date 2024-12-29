use metamodel::{next_instance_id, Entity, EntityAttrs, EntityVersion, InstanceId, UniqueId};
use chrono::{self, Utc};
use uuid::Uuid;

use aggregate_root::AggregateRoot;

#[derive(AggregateRoot)]
pub struct Album {
    // Entity data
    // TODO: Could we use another proc_macro to add these fields?
    entity_attrs: EntityAttrs,

    // Album data
    pub title: String,
    // artist: String,
    // year: i32,
    // genre: String,
    // tracks: Vec<Track>,
}

impl Album {
    fn new(entity_id: UniqueId, title: String) -> Self {
        Album {
            entity_attrs: EntityAttrs {
                id: entity_id,
                version: 0,
                discarded: false,
                instance_id: next_instance_id(),
            },
            title,
            // artist,
            // year,
            // genre,
            // tracks: Vec::new(),
        }
    }

    pub fn set_title(&mut self, title: String) {
        let event = MutationEvents::TitleUpdated {
            aggregate_id: self.id,
            aggregate_version: self.version + 1,
            timestamp: Utc::now(),
            title,
        };
        self._apply(event);
        // publish(event)
    }

    fn _apply(&mut self, event: MutationEvents) {
        mutate(event, self);
    }
}

pub enum CreationEvents {
    Created {
        // event data
        aggregate_id: UniqueId,
        timestamp: chrono::DateTime<Utc>,

        // album data
        title: String,
    },
}

pub enum MutationEvents {
    TitleUpdated {
        // event data
        aggregate_id: UniqueId,
        aggregate_version: EntityVersion,
        timestamp: chrono::DateTime<Utc>,

        // album data
        title: String,
    },
}

pub fn add_album(title: String) -> Album {
    let aggregate_id = Uuid::new_v4();
    let event = CreationEvents::Created {
        aggregate_id,
        timestamp: Utc::now(),
        title,
    };

    let album = create(event);
    // publish(event);
    album
}

fn create(event: CreationEvents) -> Album {
    match event {
        CreationEvents::Created {
            aggregate_id,
            timestamp: _,
            title,
        } => Album::new(aggregate_id, title),
    }
}

fn mutate(event: MutationEvents, album: &mut Album) {
    match event {
        MutationEvents::TitleUpdated {
            aggregate_id: _,
            aggregate_version: _,
            timestamp: _,
            title,
        } => {
            album.title = title;
        }
    }
}

