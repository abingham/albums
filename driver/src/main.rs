use albums::album::{add_album, AlbumRepository};
use albums::events::Event as AlbumEvent;
use infrastructure::{in_memory_event_store::InMemoryEventStore, unit_of_work::UnitOfWork};
use metamodel::entity::{Entity, UniqueId};
use metamodel::event::EventRouter;

fn main() {
    let event_store = InMemoryEventStore::<AlbumEvent>::empty();
    let mut event_router = EventRouter::<AlbumEvent>::new();

    // Create an album inside a unit of work and commit it
    let album_id: UniqueId;
    {
        let uow = UnitOfWork::on(&event_store);
        event_router.add_listener(Box::new(uow));
        let album = add_album("The Dark Side of the Moon".to_string(), &mut event_router);
        album_id = album.id();
        // uow.commit();
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
