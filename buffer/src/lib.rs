extern crate fte_module;

mod types;
mod buffer;
mod buffer_manager;

use ::buffer::Buffer;
use ::buffer_manager::BufferManager;

pub use ::types::Error;
pub use ::types::Result;
pub use ::types::Uuid;

pub struct Module {
    buffer_manager: BufferManager,
}

impl Module {
    pub fn new() -> Self {
        Module { buffer_manager: BufferManager::new() }
    }

    fn create_buffer(&mut self) -> Result<Uuid> {
        let buffer = Buffer::new();
        let id = buffer.id;
        try!(self.buffer_manager.insert_buffer(buffer));
        Ok(id)
    }

    pub fn delete_buffer(&mut self, uuid: &Uuid) -> Result<()> {
        try!(self.buffer_manager.remove_buffer(uuid));
        Ok(())
    }

    // create an empty buffer with optional initial content
    fn event_create(&mut self, event: fte_module::Event) -> fte_module::Result<()> {
        self.create_buffer().unwrap();
        fte_module::Module::send_event(self, event);
        Ok(())
    }

    // delete buffer
    fn event_delete(&mut self, event: fte_module::Event) -> fte_module::Result<()> {
        Ok(())
    }

    // set buffer (string or load)
    fn event_set(&mut self, event: fte_module::Event) -> fte_module::Result<()> {
        Ok(())
    }
}

impl fte_module::Module for Module {
    fn handle_event(&mut self, event: fte_module::Event) -> fte_module::Result<()> {
        match event.kind.as_str() {
            "create" => self.event_create(event),
            "delete" => self.event_delete(event),
            "set" => self.event_set(event),
            _ => Err(fte_module::Error::InvalidKind),
        }
    }
}

pub struct NewModule;

impl NewModule {
    pub fn new() -> Self {
        NewModule
    }
}

impl fte_module::NewModule for NewModule {
    type Module = Module;
    fn new_module(&self) -> Self::Module {
        Module::new()
    }
}
