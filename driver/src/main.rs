use albums::album::{Event as AlbumEvent, add_album, AlbumRepository};
use infrastructure::in_memory_event_store::InMemoryEventStore;
use metamodel::entity::{Entity, UniqueId};

fn main() {
    let event_store = InMemoryEventStore::<AlbumEvent>::empty();

    // Create an album inside a unit of work and commit it
    let album_id: UniqueId;
    {
        let uow = UnitOfWork::on(event_store);
        let album = add_album("The Dark Side of the Moon".to_string());
        album_id = album.id();
        uow.commit();
    }

    // We should now be able to fetch the album from the repo.
    match event_store.get_album_by_id(album_id) {
        Ok(album) => {
            println!("{}", album.title)
        }
        Err(err) => {
            todo!();
        }
    }
}
