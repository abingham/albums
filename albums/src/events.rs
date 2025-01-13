use metamodel::event::EventBody;

// NB: These are *all* events in the domain model.
#[derive(Clone)]
pub enum Event {
    AlbumCreated { title: String },
    AlbumTitleUpdated { title: String },
}

impl EventBody for Event {}
