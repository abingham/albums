// NB: These are *all* events in the domain model.
#[derive(Clone)]
pub enum Event {
    AlbumCreated { title: String },
    AlbumTitleUpdated { title: String },
}
