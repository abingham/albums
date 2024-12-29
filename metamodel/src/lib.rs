use uuid::Uuid;

pub type UniqueId = Uuid;
pub type InstanceId = u64;
pub type EntityVersion = u64;


pub struct EntityAttrs {
    pub id: UniqueId,
    pub version: EntityVersion,
    pub discarded: bool,
    pub instance_id: InstanceId,
}


pub trait Entity {
    // The unique ID of the entity in the domain.
    fn id(&self) -> UniqueId;

    // The unique ID of the *instance* of the Entity. This is unique among all
    // instances of the entity, even they share the same id.
    fn instance_id(&self) -> InstanceId;

    // The version of the entity. This is incremented for each change to the entity instance.
    fn version(&self) -> EntityVersion;

    fn discarded(&self) -> bool;

    // The root entity of the aggregate that this entity is a part of.
    fn aggregate_root_entity_id(&self) -> &impl Entity;
}

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
