use albums::album::{add_album, AlbumRepository};
use albums::events::Event as AlbumEvent;
use infrastructure::{in_memory_event_store::InMemoryEventStore, unit_of_work::UnitOfWork};
use metamodel::entity::{Entity, UniqueId};

fn main() {
    let mut event_store = InMemoryEventStore::<AlbumEvent>::empty();

    // Create an album inside a unit of work and commit it
    let album_id: UniqueId;
    {
        let mut uow = UnitOfWork::on(&mut event_store);
        let album = add_album("The Dark Side of the Moon".to_string());
        album_id = album.id();
        uow.commit();
    }

    // We should now be able to fetch the album from the repo.
    match event_store.get_album_by_id(album_id) {
        Ok(album) => {
            println!("{}", album.title)
        }
        Err(_err) => {
            todo!("oops");
        }
    }
}
