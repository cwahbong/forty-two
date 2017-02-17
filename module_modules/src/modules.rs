use std::collections::hash_map::{Entry, HashMap};
use std::net::SocketAddr;

use result::{Error, Result};

pub struct Inner {
    map: HashMap<String, SocketAddr>
}

impl Inner {
    pub fn new() -> Self {
        Inner { map: HashMap::new() }
    }

    pub fn regist(&mut self, name: String, addr: SocketAddr) -> Result<()> {
        match self.map.entry(name) {
            Entry::Occupied(_) => Err(Error::AlreadyRegisted),
            Entry::Vacant(entry) => {
                entry.insert(addr);
                Ok(())
            }
        }
    }

    pub fn query(&self, name: &String) -> Result<bool> {
        Ok(self.map.contains_key(name))
    }

    pub fn deregist(&mut self, name: &String) -> Result<()> {
        self.map.remove(name);
        Ok(())
    }
}
