use infrastructure::sqlite_event_store::SqliteEventStore;

use metamodel::event::{now, EventBody};

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
struct TestEvent {
    x: i32,
}

impl EventBody for TestEvent {}

fn main() {
    let mut store = SqliteEventStore::empty().expect("invalid");
    let event = now(TestEvent { x: 42 });
    store.append(event).expect("invalid");

    let events = store.iter::<TestEvent>();
    match events {
        Ok(e) => println!("{:?}", e),
        Err(e) => println!("{:?}", e),
    };

    // let event_store = InMemoryEventStore::<AlbumEvent>::empty();
    // let mut event_router = EventRouter::<AlbumEvent>::new();

    // // Create an album inside a unit of work and commit it
    // let album_id: UniqueId;
    // {
    //     let uow = UnitOfWork::on(&event_store);
    //     event_router.add_listener(Box::new(uow));
    //     let album = add_album("The Dark Side of the Moon".to_string(), &mut event_router);
    //     album_id = album.id();
    //     // uow.commit();
    // }

    // // We should now be able to fetch the album from the repo.
    // match event_store.get_album_by_id(album_id) {
    //     Ok(album) => {
    //         println!("{}", album.title)
    //     }
    //     Err(_err) => {
    //         todo!("oops");
    //     }
    // }
}
