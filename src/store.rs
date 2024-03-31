use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Local, TimeDelta};

pub struct RedisDataStore {
    shared_map: Arc<Mutex<HashMap<String, SavedItem>>>,
}

#[derive(Debug, Clone)]
pub struct SavedItem {
    pub value: String,
    pub expiration: Option<ExpiryKeeper>,
}

impl SavedItem {
    pub fn new(value: String, expiration: Option<ExpiryKeeper>) -> SavedItem {
        SavedItem {
            value: value,
            expiration: expiration,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpiryKeeper {
    pub set_time: DateTime<Local>,
    pub delta: TimeDelta,
}

impl ExpiryKeeper {
    pub fn new(set_time: DateTime<Local>, delta: TimeDelta) -> ExpiryKeeper {
        ExpiryKeeper {
            set_time: set_time,
            delta: delta,
        }
    }
}

impl RedisDataStore {
    pub fn new(hashmap: Arc<Mutex<HashMap<String, SavedItem>>>) -> Self {
        Self {
            shared_map: hashmap,
        }
    }

    pub fn handle_expirations(&mut self) {
        let mut map = self.shared_map.lock().unwrap();
        map.retain(|_, value| match &value.expiration {
            Some(expiration) => expiration.set_time + expiration.delta > Local::now(),
            None => true,
        })
    }

    pub fn set_exp(&mut self, key: String, val: String, delta: TimeDelta) -> Option<SavedItem> {
        let mut map = self.shared_map.lock().unwrap();
        let expiry_keeper = ExpiryKeeper::new(Local::now(), delta);
        let saved_item = SavedItem::new(val, Some(expiry_keeper));
        let stuff = map.insert(key, saved_item);

        stuff
    }

    pub fn set(&mut self, key: String, val: String) -> Option<SavedItem> {
        let mut map = self.shared_map.lock().unwrap();
        let saved_item = SavedItem::new(val, None);
        let stuff = map.insert(key, saved_item);

        stuff
    }

    pub fn get(&self, key: &str) -> Option<SavedItem> {
        let mut map = self.shared_map.lock().unwrap();
        let mut is_expired = false;
        if let Some(item) = map.get(key) {
            if let Some(expiration) = &item.expiration {
                if expiration.set_time + expiration.delta < Local::now() {
                    is_expired = true;
                }
            }
        }
        if is_expired {
            map.remove(key);
            None
        } else {
            map.get(key).cloned()
        }
    }
}
