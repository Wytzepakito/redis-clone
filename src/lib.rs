pub mod marshall;
pub mod responder;
pub mod store;


use marshall::{Marshaller, MessageSegment};
use responder::{Command, Responder};
use store::RedisDataStore;

pub const MAX_SIZE: usize = 30;
pub const DECIMAL_RADIX: u32 = 10;

pub struct Redis {
    pub store: RedisDataStore,
    pub marshaller: Marshaller,
    pub responder: Responder,
}



impl Redis {
    pub fn new() -> Self {
        Self {
            store: RedisDataStore::new(),
            marshaller: Marshaller {},
            responder: Responder {},
        }
    }

    pub fn make_response(&mut self, words: MessageSegment) -> Result<Command, String> {
        let array = words.get_array()?;
        let command = array[0].get_string()?;

        match command {
            "ping" => Ok(Command::PING),
            "echo" => Ok(Command::ECHO(array[1].get_string()?.to_string())),
            _ => Err(String::from("Unknown command")),
        }
    }
}

impl Redis {}

impl RedisDataStore {
    fn save(&mut self, words: Vec<String>) {}
}
