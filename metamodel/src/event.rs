use chrono::Utc;
use serde::Serialize;

use crate::entity::{EntityVersion, UniqueId};

pub trait EventBody: Clone {}

#[derive(Clone, Debug, Serialize)]
pub struct Event<T>
where
    T: EventBody,
{
    pub aggregate_id: UniqueId,
    pub aggregate_version: EntityVersion,
    pub timestamp: chrono::DateTime<Utc>,
    pub body: T,
}

impl<T: EventBody> Event<T> {
    pub fn now(aggregate_id: UniqueId, aggregate_version: EntityVersion, body: T) -> Self {
        Event {
            aggregate_id,
            aggregate_version,
            timestamp: Utc::now(),
            body,
        }
    }
}

pub fn now<E: EventBody>(body: E) -> Event<E> {
    Event {
        aggregate_id: uuid::Uuid::new_v4(),
        aggregate_version: 0,
        timestamp: Utc::now(),
        body,
    }
}

pub struct EventRouter<E: EventBody> {
    listeners: Vec<Box<dyn EventListener<E>>>,
}

impl<E: EventBody> EventRouter<E> {
    pub fn new() -> Self {
        EventRouter { listeners: vec![] }
    }

    pub fn add_listener(&mut self, listener: Box<dyn EventListener<E>>) {
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
pub trait EventListener<E: EventBody> {
    fn receive(&mut self, event: &Event<E>);
}
