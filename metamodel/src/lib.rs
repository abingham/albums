use uuid::Uuid;

use chrono::Utc;
pub type UniqueId = Uuid;
pub type InstanceId = u64;
pub type EntityVersion = u64;

pub struct Entity<T> {
    pub id: UniqueId,
    pub version: EntityVersion,
    pub discarded: bool,
    pub instance_id: InstanceId,
    pub body: T,
}

impl<T> Entity<T> {
    pub fn new(id: UniqueId, body: T) -> Self {
        Entity {
            id,
            version: 0,
            discarded: false,
            instance_id: next_instance_id(),
            body,
        }
    }
}

pub struct Event<T> {
    pub aggregate_id: UniqueId,
    pub aggregate_version: EntityVersion,
    pub timestamp: chrono::DateTime<Utc>,
    pub body: T,
}

impl<T> Event<T> {
    pub fn now(aggregate_id: UniqueId, aggregate_version: EntityVersion, body: T) -> Self {
        Event {
            aggregate_id,
            aggregate_version,
            timestamp: Utc::now(),
            body,
        }
    }
}

pub fn now<T>(body: T) -> Event<T> {
    Event {
        aggregate_id: uuid::Uuid::new_v4(),
        aggregate_version: 0,
        timestamp: Utc::now(),
        body,
    }
}


// #[derive(Debug, PartialEq)]
// pub struct BankAccountError(String);

// impl Display for BankAccountError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

pub trait AggregateRoot: Sized {
    type Event;

    fn create(event: Event<Self::Event>) -> Entity<Self> {
        let entity = Self::create_impl(&event.body);
        Entity::new(event.aggregate_id, entity)
    }

    fn create_impl(event: &Self::Event) -> Self;

    fn apply_event(entity: &mut Entity<Self>, event: &Event<Self::Event>) {
        Self::apply_event_impl(&mut entity.body, &event.body);
        entity.version += 1;
    }
    fn apply_event_impl(entity: &mut Self, event: &Self::Event);
}

// pub fn apply_event<T, E>(agg: &mut T, event: &Event<E>) where T: AggregateRoot<E>,  {
//      agg.apply_event(&event.body);
// }

// pub trait Entity {
//     // The unique ID of the entity in the domain.
//     fn id(&self) -> UniqueId;

//     // The unique ID of the *instance* of the Entity. This is unique among all
//     // instances of the entity, even they share the same id.
//     fn instance_id(&self) -> InstanceId;

//     // The version of the entity. This is incremented for each change to the entity instance.
//     fn version(&self) -> EntityVersion;

//     fn discarded(&self) -> bool;

//     // The root entity of the aggregate that this entity is a part of.
//     fn aggregate_root_entity_id(&self) -> &impl Entity;
// }

static mut NEXT_INSTANCE_ID: u64 = 0;

pub fn next_instance_id() -> u64 {
    let result: u64;
    unsafe {
        // TODO: Put a lock around this or something?
        result = NEXT_INSTANCE_ID;
        NEXT_INSTANCE_ID += 1;
    }
    return result;
}
