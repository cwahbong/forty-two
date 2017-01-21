extern crate env_logger;
extern crate fte_module;

use fte_module::api::Table;
use fte_module::types::Event;
use fte_module::types::EventArguments;
use fte_module::Module;
use fte_module::part::SendEvent;
use fte_module::part::ReceiveEvent;

struct StartModuleSendEvent;

impl SendEvent for StartModuleSendEvent {
    fn send_event(&mut self, _: Event) {}
}

struct StartModuleReceiveEvent;

impl ReceiveEvent for StartModuleReceiveEvent {
    fn receive_event(&mut self) -> Event {
        Event {
            name: String::from("end"),
            kind: String::from("module"),
            arguments: EventArguments {}
        }
    }
}

#[test]
fn start_module() {
    env_logger::init().unwrap();
    Module::start(Table::new(), StartModuleReceiveEvent, StartModuleSendEvent);
}
