extern crate fte_module;
extern crate serde;
extern crate serde_json;

use fte_module::types::{Arguments, Data};
use std::sync::{Arc, RwLock};

pub trait InnerCall {
    type Inner;
    type Args: serde::Deserialize;
    type Rets: serde::Serialize;
    type Error: serde::Serialize;

    fn run(inner: &Arc<RwLock<Self::Inner>>, args: Self::Args) -> std::result::Result<Self::Rets, Self::Error>;
}

pub struct Call<IC: InnerCall> {
    inner: Arc<RwLock<IC::Inner>>,
}

impl <IC: InnerCall> Call <IC> {
    pub fn new(inner: Arc<RwLock<IC::Inner>>) -> Self {
        Call { inner: inner }
    }
}

impl <IC: InnerCall> fte_module::api::Call for Call <IC> {
    fn call(&self, arguments: Arguments) -> std::result::Result<Data, fte_module::types::Error> {
        IC::run(&self.inner, serde_json::from_value(arguments)?)
            .and_then(|v| Ok(serde_json::to_value(v).unwrap()))
            .map_err(|e| serde_json::to_value(e).unwrap().into())
    }
}

#[macro_export]
macro_rules! inner_map_api {
    ($inner:expr,$($name:expr => $inner_call:path),*) => {{
        let rwlock_inner = std::sync::Arc::new(std::sync::RwLock::new($inner));
        let mut map = std::collections::HashMap::<String, Box<fte_module::api::Call>>::new();
        $(map.insert($name.into(), Box::new($crate::Call::<$inner_call>::new(rwlock_inner.clone())));)*
        fte_module::api::MapApi::new(map)
    }};
    ($inner:expr,$($name:expr => $inner_call:path),*,) => {
        inner_map_api!($inner, $($name => $inner_call),*)
    };
}
