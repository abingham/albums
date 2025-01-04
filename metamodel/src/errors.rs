use crate::entity::UniqueId;

pub struct NoSuchEntityError {
    id: UniqueId
}

impl NoSuchEntityError {
    pub fn new(id: UniqueId) -> Self {
        NoSuchEntityError { id }
    }
    pub fn id(&self) -> UniqueId {
        self.id
    }
}