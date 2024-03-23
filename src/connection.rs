use std::{
    collections::HashMap,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    config::Config, formatter::{make_fullresync_str, make_info_str, make_simple_str}, marshall::Marshaller, responder::{Command, Responder}, store::RedisDataStore
};

pub struct Connection {
    pub store: RedisDataStore,
    pub marshaller: Marshaller,
    pub responder: Responder,
    pub config: Config,
}

impl Connection {
    pub fn new(store: RedisDataStore, config: Config) -> Self {
        Self {
            store: store,
            marshaller: Marshaller {},
            responder: Responder {},
            config: config,
        }
    }

    pub fn process_stream(&mut self, stream: &mut TcpStream) -> Result<String, String> {
        let words = self.marshaller.parse_redis_command(stream);
        println!("words received: {:?}", words);
        let optional_command = self
            .marshaller
            .make_command(words.expect("Couldn't get words"));
        let command = optional_command.expect("Couldn't get command");
        self.process_command(&command)
    }

    pub fn process_command(&mut self, command: &Command) -> Result<String, String> {
        match command {
            Command::PING => Ok(format!("+PONG\r\n")),
            Command::PONG => Ok(String::from("")),
            Command::OK => Ok(String::from("")),
            Command::FULLRESYNC => Ok(String::from("")),
            Command::ECHO(msg) => Ok(format!("${}\r\n{}\r\n", msg.len(), msg)),
            Command::SET(key, val) => {
                self.store
                    .set(key.to_string(), val.to_string())
                    .map(|_| println!("Key was already present in store"));
                Ok(format!("+OK\r\n"))
            }
            Command::SET_EXP(key, val, delta) => {
                self.store
                    .set_exp(key.to_string(), val.to_string(), delta.clone())
                    .map(|_| println!("Key was already present in store"));
                Ok(format!("+OK\r\n"))
            }
            Command::GET(key) => {
                let result = self.store.get(key);
                match result {
                    Some(saved) => Ok(format!("${}\r\n{}\r\n", saved.value.len(), saved.value)),
                    None => Ok(format!("$-1\r\n")),
                }
            }
            Command::INFO(info_command) => Ok(make_info_str(&self.config)),
            Command::REPLCONF => Ok(format!("+OK\r\n")),
            Command::PSYNC => Ok(make_fullresync_str(&self.config)),
        }
    }

    pub fn handle_stream(&mut self, mut stream: TcpStream) {
        loop {
            // Process the received data (you can replace this with your own logic)
            let response: Result<String, String> = self.process_stream(&mut stream);
            // Write back to the TcpStream
            stream
                .write_all(response.expect("Couldn't get response").as_bytes())
                .unwrap();
        }
    }
}
