use metamodel::event::EventListener;

use crate::in_memory_event_store::InMemoryEventStore;

pub struct UnitOfWork<E: Clone> {
    transient_events: InMemoryEventStore<E>,
    event_store: InMemoryEventStore<E>
}

impl<E: Clone> UnitOfWork<E> {
    pub fn on(store: &InMemoryEventStore<E>) -> Self {
        UnitOfWork {
            transient_events: InMemoryEventStore::<E>::empty(),
            event_store: store.clone(),
        }
    }

    pub fn commit(&mut self) {
        // Copy the events in the transient store into the permanent store.
        for event in self.transient_events.iter() {
            self.event_store.append(event.clone());
        }
    }
}

impl<E: Clone> EventListener<E> for UnitOfWork<E> {
    fn receive(&mut self, event: &metamodel::event::Event<E>) {
        self.transient_events.append(event.clone());
    }
}
