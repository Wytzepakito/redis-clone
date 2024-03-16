use std::{collections::HashMap, sync::{Arc, Mutex}};

pub struct RedisDataStore {
    shared_map: Arc<Mutex<HashMap<String, String>>>,
}

impl RedisDataStore {
    pub fn new(hashmap: Arc<Mutex<HashMap<String, String>>>) -> Self {
        Self {
            shared_map: hashmap,
        }
    }

    pub fn set(&mut self, key: String, val: String) -> Option<String> {
        println!("Inserting {} into {}", val, key);
        let mut map = self.shared_map.lock().unwrap();
        let stuff = map.insert(key, val);

        stuff
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let mut map = self.shared_map.lock().unwrap();
        let result = map.get(key);
        result.cloned()
    }
}
