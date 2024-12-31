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

impl EntityAttrs {
    pub fn new() -> Self {
        EntityAttrs {
            id: uuid::Uuid::new_v4(),
            version: 0,
            discarded: false,
            instance_id: next_instance_id(),
        }
    }
}

pub trait Entity {
    fn id(&self) -> UniqueId;
    fn version(&self) -> EntityVersion;
    fn inc_version(&mut self);
    fn discarded(&self) -> bool;
    fn instance_id(&self) -> InstanceId;
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
