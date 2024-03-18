use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use clap::{arg, command, value_parser};
use redis_starter_rust::config::{Config, Role};
use redis_starter_rust::store::RedisDataStore;
use redis_starter_rust::Redis;

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

fn get_port() -> u32 {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -p --port <int> "Sets a custom port number"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(u32)),
        )
        .get_matches();
    let port = match matches.get_one::<u32>("port") {
        Some(port) => match port {
            x if x > &9999 => 6379 as u32,
            x => *x,
        },
        None => 6379 as u32,
    };
    println!("port is: {:?}", port);
    port
}

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let port = get_port();

    let listener = TcpListener::bind(format!("127.0.0.1:{:0>4}", port)).unwrap();
    let hashmap = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        let store = RedisDataStore::new(hashmap.clone());
        let config = Config::new(Role::MASTER);
        let redis = Redis::new(store, config);
        thread::spawn(move || {
            //handle_expirations(&mut redis);
            handle_stream(stream.unwrap(), redis);
        });
    }

    Ok(())
}
