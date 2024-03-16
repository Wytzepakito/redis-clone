use std::collections::HashMap;

pub struct RedisDataStore {
    map: HashMap<String, String>,
}

impl RedisDataStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
