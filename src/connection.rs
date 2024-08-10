use std::{io::{BufReader, Read, Write}, net::TcpStream, sync::{Arc, Mutex}, thread, time::Duration};

use crate::{
    config::Config, formatter::{make_bulk_str, make_fullresync_str, make_info_str}, marshall::Marshaller, responder::{Command, Responder}, store::RedisDataStore
};

pub struct Connection {
    pub store: RedisDataStore,
    pub marshaller: Marshaller,
    pub responder: Responder,
    pub config: Config,
    pub receiving_from_master: Arc<Mutex<bool>>
}

impl Connection {
    pub fn new(store: RedisDataStore, config: Config) -> Self {
        Self {
            store: store,
            marshaller: Marshaller {},
            responder: Responder {},
            config: config,
            receiving_from_master: Arc::new(Mutex::new(false))
        }
    }


    pub fn handle_stream(&mut self, mut stream: &TcpStream) {
        loop {
            let words = self.marshaller.parse_redis_command(stream);
            println!("words received: {:?}", words);
            let optional_command = self
                .marshaller
                .make_command(words.expect("Couldn't get words"));
            let command = optional_command.expect("Couldn't get command");

            self.process_command(&command, stream);

            thread::sleep(Duration::from_millis(125));
        }
    }

    pub fn process_command(
        &mut self,
        command: &Command,
        stream: &TcpStream,
    )  {
        match command {
            Command::PING => self.responder.send_pong_response(stream),
            Command::PONG => self.responder.send_empty_response(stream),
            Command::OK => self.responder.send_empty_response(stream),
            Command::FULLRESYNC => self.responder.send_empty_response(stream),
            Command::ECHO(msg) => self.responder.send_string_response(stream, msg.to_string()),
            Command::SET(key, val) => {
                println!("Setting {} to {}", key.to_string(), val.to_string());
                self.store
                    .set(key.to_string(), val.to_string())
                    .map(|_| println!("Key was already present in store"));
                if let Some(master) = self.config.get_master() {
                    println!("Propogating command: {:?}", command);
                    master.propagate_commands(self.responder.copy_request(command))
                };
                self.responder.send_ok_reponse(stream)
            }
            Command::SETEXP(key, val, delta) => {
                self.store
                    .set_exp(key.to_string(), val.to_string(), delta.clone())
                    .map(|_| println!("Key was already present in store"));
                self.responder.send_ok_reponse(stream)
            }
            Command::GET(key) => {
                let result = self.store.get(key);

                match result {
                    Some(saved) => self.responder.send_string_response(stream, saved.value),
                    None => self.responder.send_empty_get_reponse(stream),
                }
            }
            Command::INFO(_) => self.responder.send_vec_response(stream, make_info_str(&self.config)),
            Command::REPLCONF(_) => self.responder.send_ok_reponse(stream),
            Command::PSYNC => {
                if let Some(master) = self.config.get_master() {
                    let mut locked_streams = master.streams.lock().unwrap();
                    locked_streams.push(stream.try_clone().unwrap());

                };
                self.responder.send_vec_response(stream, make_fullresync_str(&self.config));
            }
        }
    }



    pub fn handle_master_stream(&mut self, mut master_stream: TcpStream) {
        loop {
            let _ = self.process_master_stream( &mut master_stream);
            thread::sleep(Duration::from_millis(125));
        }
    }

    fn process_master_stream(&mut self, master_stream: &TcpStream)  {
        let words = self.marshaller.parse_redis_command(master_stream);
        println!("Processing master stream, words: {:?}", words);
        
        println!("words received: {:?}", words);
        let optional_command = self
            .marshaller
            .make_command(words.expect("Couldn't get words"));
        let command = optional_command.expect("Couldn't get command");

        self.process_command(&command, master_stream)

    }
}

