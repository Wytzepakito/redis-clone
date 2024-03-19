
pub mod config;
pub mod connection;
pub mod marshall;
pub mod responder;
pub mod store;
use std::{
    collections::HashMap,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use config::{Config, SlaveConfig};

use crate::config::{get_config, Role};

use crate::responder::Responder; 
use crate::connection::Connection;
use crate::store::RedisDataStore;


pub struct RedisServer {
}

impl RedisServer {
    pub fn new() -> RedisServer {
        RedisServer {}
    }

    fn slave_handshake(&self, mut master_stream: TcpStream, replicated_port: &u32) {
        let mut responder = Responder::new();
        master_stream
            .write_all(responder.ping_request().as_bytes())
            .unwrap();
        println!("Send ping");
        
        master_stream
            .write_all(responder.replconf_request_one(replicated_port).as_bytes())
            .unwrap();
        master_stream
            .write_all(responder.replconf_request_two().as_bytes())
            .unwrap();
    }

    fn spawn_slave(&mut self, config: Config) {


        let listener = TcpListener::bind(format!("127.0.0.1:{:0>4}", &config.port)).unwrap();
        let replicated_port = 
            &config
                .role
                .get_slave_config()
                .expect("Slave config should be there")
                .replicated_port;
        let master_stream = TcpStream::connect(format!(
            "127.0.0.1:{:0>4}",
            replicated_port
        ))
        .unwrap();
        let hashmap = Arc::new(Mutex::new(HashMap::new()));


        self.slave_handshake(master_stream, &config.port);

        while let (Ok((stream, _))) = listener.accept() {

            let store = RedisDataStore::new(hashmap.clone());
            let mut redis = Connection::new(store, config.clone());
            thread::spawn(move || {
                //handle_expirations(&mut redis);
                redis.handle_stream(stream);
            });
        }
    }

    fn spawn_master(&mut self, config: Config) {
        let listener = TcpListener::bind(format!("127.0.0.1:{:0>4}", config.port)).unwrap();
        let hashmap = Arc::new(Mutex::new(HashMap::new()));

        while let (Ok((stream, _))) = listener.accept() {

            let store = RedisDataStore::new(hashmap.clone());
            let mut redis = Connection::new(store, config.clone());
            thread::spawn(move || {
                //handle_expirations(&mut redis);
                redis.handle_stream(stream);
            });
        }
    }


    pub fn run(&mut self) {

        let config = get_config();

        match config.role {
            Role::MASTER(_) => self.spawn_master(config),
            Role::SLAVE(_) => self.spawn_slave(config),
        }
    }
}
