use metamodel::event::EventListener;

use crate::in_memory_event_store::InMemoryEventStore;

pub struct UnitOfWork<'a, E: Clone> {
    transient_events: InMemoryEventStore<E>,
    event_store: &'a mut InMemoryEventStore<E>,
}

impl<'a, E: Clone> UnitOfWork<'a, E> {
    pub fn on(store: &'a mut InMemoryEventStore<E>) -> Self {
        UnitOfWork {
            transient_events: InMemoryEventStore::<E>::empty(),
            event_store: store,
        }
    }

    pub fn commit(&mut self) {
        // Copy the events in the transient store into the permanent store.
        for event in self.transient_events.events() {
            self.event_store.append(event.clone());
        }
    }
}

impl<'a, E: Clone> EventListener<E> for UnitOfWork<'a, E> {
    fn receive(&mut self, event: &metamodel::event::Event<E>) {
        self.transient_events.append(event.clone());
    }
}
