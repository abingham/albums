use albums::album::{Album, AlbumRepository, Event as AlbumEvent};
use metamodel::{entity::UniqueId, errors::NoSuchEntityError};
use metamodel::aggregate_root::AggregateRoot;

use crate::in_memory_event_store::InMemoryEventStore;

impl AlbumRepository for InMemoryEventStore<AlbumEvent> {
    fn get_album_by_id(&self, id: UniqueId) -> Result<albums::album::Album, NoSuchEntityError> {
        let mut maybe_album: Option<Album> = None;
        for event in self.events() {
            match maybe_album {
                Some(ref mut album) => {
                    album.apply_event(&event);
                },
                None => {
                    maybe_album = Some(Album::create(&event));
                },
            }
        }

        match maybe_album {
            None => Err(NoSuchEntityError::new(id)),
            Some(album) => Ok(album)
        }
    }
    
    fn put(&self, _id: UniqueId) {
        todo!()
    }
}
