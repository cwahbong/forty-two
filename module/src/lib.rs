use ::types::Result;
pub use ::types::Event;

pub mod functions;
pub mod types;
pub mod service;

// Need a sender for high level event communication
//
// Regist
// * Function
// * Stream Receiver
//
// Sender for
// * Remote function call
// * Stream Sender

pub trait Module {
    fn send_event(&mut self, event: Event) -> Result<()> {
        // TODO
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<()>;
}

pub trait NewModule {
    type Module: Module;
    fn new_module(&self) -> Self::Module;
}

pub fn main<N: NewModule>(new_module: N) {
    // TODO make use of this module object
}
