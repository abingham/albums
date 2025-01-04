use metamodel::event::Event;

pub struct InMemoryEventStore<E> {
    events: Vec<Event<E>>   
}

impl<E> InMemoryEventStore<E> {
    pub fn empty() -> Self {
        InMemoryEventStore { events: vec![] }
    }

    pub fn events(&self) -> &Vec<Event<E>> {
        &self.events
    }
}