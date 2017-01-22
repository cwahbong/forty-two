use ::functions::Call;
use ::types::Event;
use ::types::Result;
use ::Module;

pub struct Service {
    pub calls: Vec<Box<Call>>,
    pub events: Vec<Box<Event>>,
    // streams: Stream,
}

impl Service {
    pub fn new() -> Self {
        Service {
            calls: Vec::new(),
            events: Vec::new(),
        }
    }
}

pub struct ServiceModule {
}

impl ServiceModule {
    pub fn new(service: Service) -> Self {
        // TODO
        ServiceModule {}
    }
}

// impl Module for ServiceModule {
//     fn handle_event(&mut self, event: Event) -> Result<()> {
//         Ok(())
//     }
// }
