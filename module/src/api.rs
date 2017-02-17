use serde_json;
use std::collections::HashMap;

use types::{Arguments, Data, Error, RequestEvent, ResponseEvent};

pub trait Api {
    fn run(&self, request: RequestEvent) -> ResponseEvent;
}

pub trait Call {
    fn call(&self, arguments: Arguments) -> Result<Data, Error>;
    fn run(&self, arguments: Arguments) -> ResponseEvent {
        match self.call(arguments) {
            Ok(data) => ResponseEvent {
                success: true,
                data: data
            },
            Err(error) => ResponseEvent {
                success: false,
                data: serde_json::to_value(error).unwrap()
            }
        }
    }
}

pub struct MapApi {
    api_map: HashMap<String, Box<Call>>,
}

impl MapApi {
    pub fn new(api_map: HashMap<String, Box<Call>>) -> Self {
        MapApi { api_map: api_map }
    }
}

impl Api for MapApi {
    fn run(&self, request: RequestEvent) -> ResponseEvent {
        if let Some(call) = self.api_map.get(&request.name) {
            call.run(request.arguments)
        } else {
            ResponseEvent {
                success: false,
                data: serde_json::Value::Null
            }
        }
    }
}

pub trait NewApi {
    type Api: Api;
    fn new_api() -> Self::Api;
}
