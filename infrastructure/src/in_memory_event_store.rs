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

    // TODO: Should be an iterator?
    pub fn events(&self) -> &Vec<Event<E>> {
        &self.events
    }

    pub fn append(&mut self, event: Event<E>) {
        self.events.push(event);
    }
}
