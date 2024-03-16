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

    pub fn set(&mut self, key: String, val: String) -> Option<String> {
        let stuff = self.map.insert(key, val);

        println!("{:?}", stuff.as_ref().unwrap());
        stuff
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
}
