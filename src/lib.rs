pub mod config;
pub mod connection;
pub mod formatter;
pub mod marshall;
pub mod responder;
pub mod store;
pub mod model;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use config::Config;
use formatter::{make_array_str, make_bulk_str, make_simple_str};
use marshall::MessageSegment;
use model::resync_response::ResyncResponse;
use responder::Command;

use crate::{
    config::{get_config, Role},
    marshall::Marshaller,
};

use crate::connection::Connection;
use crate::responder::Responder;
use crate::store::RedisDataStore;

pub struct RedisServer {}

impl RedisServer {
    pub fn new() -> RedisServer {
        RedisServer {}
    }

    pub fn run(&mut self) {
        let config = get_config();

        match config.role {
            Role::MASTER(_) => self.spawn_master(config),
            Role::SLAVE(_) => self.spawn_slave(config),
        }
    }

    fn spawn_slave(&mut self, config: Config) {
        let listener = TcpListener::bind(format!("127.0.0.1:{:0>4}", &config.port)).unwrap();
        let replicated_port = &config
            .role
            .get_slave()
            .expect("Slave config should be there")
            .replicated_port;
        let hashmap = Arc::new(Mutex::new(HashMap::new()));
        let mut master_stream =
            TcpStream::connect(format!("127.0.0.1:{:0>4}", replicated_port)).unwrap();
        self.slave_handshake(&mut master_stream, &config.port)
            .unwrap();

        let store = RedisDataStore::new(hashmap.clone());
        println!("Store instantiated");
        let mut master_redis = Connection::new(store, config.clone());
        println!("master_redis instantiated");
        println!("slave discard rdb ran");
        thread::spawn(move || {
            master_redis.handle_master_stream(master_stream);
        });
        println!("Master stream thread spawned");

        while let Ok((stream, _)) = listener.accept() {
            let store = RedisDataStore::new(hashmap.clone());
            let mut redis = Connection::new(store, config.clone());
            thread::spawn(move || {
                //handle_expirations(&mut redis);
                redis.handle_stream(&stream);
            });
        }
    }

    fn slave_handshake(
        &self,
        master_stream: &mut TcpStream,
        replicated_port: &u32,
    ) -> Result<(), String> {
        let responder = Responder::new();
        self.send_and_ack(master_stream, make_array_str(vec![make_bulk_str(String::from("ping"))]), Command::PONG)?;

        self.send_and_ack(
            master_stream,
            responder.replconf_request_one(replicated_port),
            Command::OK,
        )?;

        self.send_and_ack(master_stream, responder.replconf_request_two(), Command::OK)?;

        self.send_and_ack(
            master_stream,
            responder.psync_request(),
            Command::FULLRESYNC,
        )?;
        Ok(())
    }

    fn send_and_ack(
        &self,
        master_stream: &mut TcpStream,
        request: Vec<u8>,
        expected_response: Command,
    ) -> Result<(), String> {
        let marshall = Marshaller::new();
        master_stream.write_all(&request).unwrap();
        if &expected_response == &Command::FULLRESYNC {
            let words: Result<ResyncResponse, String> = marshall.parse_resync(&master_stream);
            println!("{:?}", words);
            Ok(())
        } else {
            let words = marshall.parse_redis_command(&master_stream);
            let command = marshall.make_command(words?)?;

            if &command == &expected_response {
                Ok(())
            } else {
                Err(format!(
                    "Received unexpected command on {}",
                    String::from_utf8(request).unwrap()
                ))
            }
        }
    }

    fn spawn_master(&mut self, config: Config) {
        println!("Running on: {}", config.port);
        let listener = TcpListener::bind(format!("127.0.0.1:{:0>4}", config.port)).unwrap();
        let hashmap = Arc::new(Mutex::new(HashMap::new()));

        while let Ok((stream, _)) = listener.accept() {
            let store = RedisDataStore::new(hashmap.clone());
            let mut redis = Connection::new(store, config.clone());
            thread::spawn(move || {
                //handle_expirations(&mut redis);
                redis.handle_stream(&stream);
            });
        }
    }
}
