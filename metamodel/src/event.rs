use chrono::Utc;

use crate::entity::{EntityVersion, UniqueId};

#[derive(Clone)]
pub struct Event<T> where T: Clone{
    pub aggregate_id: UniqueId,
    pub aggregate_version: EntityVersion,
    pub timestamp: chrono::DateTime<Utc>,
    pub body: T,
}

impl<T: Clone> Event<T> {
    pub fn now(aggregate_id: UniqueId, aggregate_version: EntityVersion, body: T) -> Self {
        Event {
            aggregate_id,
            aggregate_version,
            timestamp: Utc::now(),
            body,
        }
    }
}

pub fn now<E: Clone>(body: E) -> Event<E> {
    Event {
        aggregate_id: uuid::Uuid::new_v4(),
        aggregate_version: 0,
        timestamp: Utc::now(),
        body,
    }
}

pub struct EventRouter<'a, E: Clone> {
    listeners: Vec<&'a mut dyn EventListener<E>>
}

impl<'a, E: Clone> EventRouter<'a, E> {
    pub fn new() -> Self {
        EventRouter {
            listeners: vec![]
        }
    }

    pub fn add_listener(&mut self, listener: &'a mut dyn EventListener<E>) {
        self.listeners.push(listener);
    }

    pub fn publish(&mut self, event: &Event<E>) {
        for listener in &mut self.listeners {
            listener.receive(event);
        }
    }
}

// TODO: I only added this because I coulnd't sort out the
// more general "callable" syntax on the plane. Replace this
// with something better soon.
pub trait EventListener<E: Clone> {
    fn receive(&mut self, event: &Event<E>);
}
