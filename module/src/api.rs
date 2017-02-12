use types::{RequestEvent, ResponseEvent};

pub trait Api {
    fn run(&self, request: RequestEvent) -> ResponseEvent;
}

pub trait NewApi {
    type Api: Api;
    fn new_api() -> Self::Api;
}
