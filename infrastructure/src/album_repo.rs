use albums::album::{Album, AlbumRepository};
use albums::events::Event as AlbumEvent;
use metamodel::aggregate_root::AggregateRoot;
use metamodel::{entity::UniqueId, errors::NoSuchEntityError};

use crate::in_memory_event_store::InMemoryEventStore;

impl AlbumRepository for InMemoryEventStore<AlbumEvent> {
    fn get_album_by_id(&self, id: UniqueId) -> Result<albums::album::Album, NoSuchEntityError> {
        let mut maybe_album: Option<Album> = None;
        for event in self.iter() {
            match maybe_album {
                Some(ref mut album) => {
                    album.apply_event(&event);
                }
                None => {
                    maybe_album = Some(Album::create(&event));
                }
            }
        }

        match maybe_album {
            None => Err(NoSuchEntityError::new(id)),
            Some(album) => Ok(album),
        }
    }

    fn put(&self, _id: UniqueId) {
        todo!()
    }
}
