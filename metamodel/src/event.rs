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

pub fn now<T: Clone>(body: T) -> Event<T> {
    Event {
        aggregate_id: uuid::Uuid::new_v4(),
        aggregate_version: 0,
        timestamp: Utc::now(),
        body,
    }
}

