use std::collections::HashMap;
use ::buffer::Buffer;
use ::types::Error;
use ::types::Result;
use ::types::Uuid;

pub struct BufferManager {
    buffers: HashMap<Uuid, Buffer>,
}

impl BufferManager {
    pub fn new() -> Self {
        BufferManager { buffers: HashMap::new() }
    }

    pub fn insert_buffer(&mut self, buffer: Buffer) -> Result<()> {
        if buffer.id.is_nil() {
            return Err(Error::UuidInvalid);
        }
        if self.buffers.contains_key(&buffer.id) {
            return Err(Error::UuidCollision);
        }
        self.buffers.insert(buffer.id.clone(), buffer);
        Ok(())
    }

    pub fn remove_buffer(&mut self, uuid: &Uuid) -> Result<Buffer> {
        self.buffers
            .remove(uuid)
            .ok_or(Error::UuidNotExists)
    }

    pub fn get_buffer(&self, uuid: &Uuid) -> Result<&Buffer> {
        self.buffers
            .get(uuid)
            .ok_or(Error::UuidNotExists)
    }

    pub fn get_mut_buffer(&mut self, uuid: &Uuid) -> Result<&mut Buffer> {
        self.buffers
            .get_mut(uuid)
            .ok_or(Error::UuidNotExists)
    }

    pub fn contains_buffer(&self, uuid: &Uuid) -> bool {
        self.buffers.contains_key(uuid)
    }
}
