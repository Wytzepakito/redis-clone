use std::{io::{BufReader, Read, Write}, net::TcpStream, sync::{Arc, Mutex}, thread, time::Duration};

use crate::{
    config::Config, formatter::{make_bulk_str, make_fullresync_str, make_info_str}, lines_marshall::{self, LinesMarshaller}, marshall::Marshaller, responder::{Command, Responder}, store::RedisDataStore
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

    pub fn process_stream(&mut self, stream: &TcpStream) -> Result<Vec<u8>, String> {
        let words = self.marshaller.parse_redis_command(stream);
        println!("words received: {:?}", words);
        let optional_command = self
            .marshaller
            .make_command(words.expect("Couldn't get words"));
        let command = optional_command.expect("Couldn't get command");

        self.process_command(&command, stream)
    }

    pub fn process_command(
        &mut self,
        command: &Command,
        stream: &TcpStream,
    ) -> Result<Vec<u8>, String> {
        match command {
            Command::PING => Ok(self.responder.pong_response()),
            Command::PONG => Ok(self.responder.empty_response()),
            Command::OK => Ok(self.responder.empty_response()),
            Command::FULLRESYNC => Ok(self.responder.empty_response()),
            Command::ECHO(msg) => Ok(make_bulk_str(msg.to_string())),
            Command::SET(key, val) => {
                println!("Setting {} to {}", key.to_string(), val.to_string());
                self.store
                    .set(key.to_string(), val.to_string())
                    .map(|_| println!("Key was already present in store"));
                if let Some(master) = self.config.get_master() {
                    println!("Propogating command: {:?}", command);
                    master.propagate_commands(self.responder.copy_request(command))
                };
                Ok(self.responder.ok_reponse())
            }
            Command::SETEXP(key, val, delta) => {
                self.store
                    .set_exp(key.to_string(), val.to_string(), delta.clone())
                    .map(|_| println!("Key was already present in store"));
                Ok(self.responder.ok_reponse())
            }
            Command::GET(key) => {
                let result = self.store.get(key);

                match result {
                    Some(saved) => Ok(make_bulk_str(saved.value)),
                    None => Ok(self.responder.empty_get_reponse()),
                }
            }
            Command::INFO(_) => Ok(make_info_str(&self.config)),
            Command::REPLCONF(_) => Ok(self.responder.ok_reponse()),
            Command::PSYNC => {
                if let Some(master) = self.config.get_master() {
                    let mut locked_streams = master.streams.lock().unwrap();
                    locked_streams.push(stream.try_clone().unwrap());
                };
                Ok(make_fullresync_str(&self.config))
            }
        }
    }

    pub fn handle_stream(&mut self, mut stream: TcpStream) {
        loop {
            // Process the received data (you can replace this with your own logic)
            let response: Result<Vec<u8>, String> = self.process_stream(&mut stream);

            // Write back to the TcpStream
            stream
                .write_all(&response.expect("Couldn't get response"))
                .unwrap();
            thread::sleep(Duration::from_millis(125));
        }
    }

    pub fn handle_master_stream(&mut self, mut master_stream: TcpStream) {
        let lines_marshall = LinesMarshaller::new();
        loop {
            let _ = self.process_master_stream(&lines_marshall, &mut master_stream);
        }
    }

    fn process_master_stream(&mut self, lines_marshall: &LinesMarshaller, master_stream: &TcpStream)  {
        self.receiving_from_master.lock();
        let mut reader = BufReader::new(master_stream);
        let mut bulk_read = String::new();

        reader.read_to_string(&mut bulk_read);


        let message_segments = lines_marshall.parse_master_commands(bulk_read);
        for segment in message_segments {

            let optional_command = self
                .marshaller
                .make_command(segment.expect("Couldn't get words"));
            let command = optional_command.expect("Couldn't get command");

            self.process_command(&command, master_stream);      
        }
        
    }
}
