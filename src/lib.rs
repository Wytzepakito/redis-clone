pub mod marshall;
pub mod responder;
pub mod store;

use std::{net::TcpStream, sync::{Arc, Mutex}};

use marshall::Marshaller;
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
    pub fn new(store: RedisDataStore) -> Self {
        Self {
            store: store,
            marshaller: Marshaller {},
            responder: Responder {},
        }
    }

    pub fn process_stream(&mut self, stream: &mut TcpStream) -> Result<String, String> {
        let words = self.marshaller.parse_redis_command(stream);
        let optional_command = self
            .marshaller
            .make_command(words.expect("Couldn't get words"));
        let command = optional_command.expect("Couldn't get command");
        self.process_command(&command)
    }

    pub fn process_command(&mut self, command: &Command) -> Result<String, String> {
        match command {
            Command::PING => Ok(format!("+PONG\r\n")),
            Command::ECHO(msg) => Ok(format!("${}\r\n{}\r\n", msg.len(), msg)),
            Command::SET(key, val) => {
                let result = self.store.set(key.to_string(), val.to_string());
                match result {
                    Some(_) => {
                        println!("key was already present in store");
                        Ok(format!("+OK\r\n"))
                    },
                    None => Ok(format!("+OK\r\n"))
                }
            }
            Command::GET(key) => {
                let result = self.store.get(key);
                match result {
                    Some(val) => Ok(format!("${}\r\n{}\r\n", val.len(), val)),
                    None => Ok(format!("$-1\r\n")),
                }
            }
        }
    }
}

impl Redis {}
