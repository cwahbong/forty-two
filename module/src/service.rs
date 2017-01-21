use ::functions::Call;
use ::types::Event;
use ::types::Result;
use ::Module;

pub struct Service {
    calls: Vec<Box<Call>>,
    events: Vec<Box<Event>>,
    // streams: Stream,
}

pub struct ServiceModule {
}

impl ServiceModule {
    pub fn new(service: Service) -> Self {
        // TODO
        ServiceModule {}
    }
}

impl Module for ServiceModule {
    fn handle_event(&mut self, event: Event) -> Result<()> {
        Ok(())
    }
}
