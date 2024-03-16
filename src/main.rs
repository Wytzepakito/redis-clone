use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use clap::Parser;
use redis_starter_rust::store::RedisDataStore;
use redis_starter_rust::Redis;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 6379)]
    port: u32
}

fn handle_stream(mut stream: TcpStream, mut redis: Redis) {
    loop {
        // Process the received data (you can replace this with your own logic)
        let response: Result<String, String> = redis.process_stream(&mut stream);

        // Write back to the TcpStream
        stream
            .write_all(response.expect("Couldn't get response").as_bytes())
            .unwrap();
    }
}

fn handle_expirations(redis: &mut Redis) {
    redis.store.handle_expirations();
}

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let mut args = Args::parse();
    if args.port > 9999 {
        println!("Port can't be higher than 9999");
        args.port = 6379;
    }
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).unwrap();
    let hashmap = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        let store = RedisDataStore::new(hashmap.clone());
        let mut redis = Redis::new(store);
        thread::spawn(move || {
            //handle_expirations(&mut redis);
            handle_stream(stream.unwrap(), redis);
        });
    }

    Ok(())
}
