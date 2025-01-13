use crate::{entity::Entity, event::Event};
use crate::event::EventBody;

pub trait AggregateRoot: Entity + Sized {
    type Event: EventBody;

    fn create(event: &Event<Self::Event>) -> Self {
        Self::create_impl(&event.body)
    }

    fn create_impl(event: &Self::Event) -> Self;

    fn apply_event(&mut self, event: &Event<Self::Event>) {
        self.apply_event_impl(&event.body);
        self.inc_version();
    }
    fn apply_event_impl(&mut self, event: &Self::Event);
}