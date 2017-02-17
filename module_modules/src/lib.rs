extern crate fte_module;
#[macro_use]
extern crate fte_rwlock_module;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod api;
pub mod modules;
pub mod result;

pub struct Modules;

impl fte_module::api::NewApi for Modules {
    type Api = fte_module::api::MapApi;
    fn new_api() -> Self::Api {
        inner_map_api!(modules::Inner::new(),
            "regist" => api::regist::InnerCall,
            "query" => api::query::InnerCall,
            "deregist" => api::deregist::InnerCall,
        )
    }
}
