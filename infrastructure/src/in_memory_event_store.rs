use metamodel::event::{Event, EventListener};

pub struct InMemoryEventStore<E: Clone> {
    events: Vec<Event<E>>   
}

impl<E: Clone> InMemoryEventStore<E> {
    pub fn empty() -> Self {
        InMemoryEventStore { events: vec![] }
    }

    pub fn events(&self) -> &Vec<Event<E>> {
        &self.events
    }

    pub fn append(&mut self, event: Event<E>) {
        self.events.push(event);
    }
}

impl<E: Clone> EventListener<E> for InMemoryEventStore<E> {
    fn receive(&mut self, event: &Event<E>) {
        self.events.push(event.clone());
    }
}