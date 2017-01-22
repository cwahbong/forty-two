extern crate fte_module;

use fte_module::service::Service;
use fte_module::service::ServiceModule;

#[test]
fn call_service_module() {
    let mut service = Service::new();
    //
    let service_module = ServiceModule::new(service);
}
