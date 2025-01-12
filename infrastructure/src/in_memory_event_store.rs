use std::slice::Iter;

use metamodel::event::Event;

#[derive(Clone)]
pub struct InMemoryEventStore<E: Clone> {
    events: Vec<Event<E>>,
}

impl<E: Clone> InMemoryEventStore<E> {
    pub fn empty() -> Self {
        InMemoryEventStore {
            events: vec![],
        }
    }

    pub fn append(&mut self, event: Event<E>) {
        self.events.push(event);
    }

    pub fn iter(&self) -> Iter<Event<E>> {
        self.events.iter()
    }
}
