use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

use redis_starter_rust::Redis;

fn handle_stream(mut stream: TcpStream) {
    loop {
        // Process the received data (you can replace this with your own logic)
        let mut redis = Redis::new();
        let response: Result<String, String> = redis.process_stream(&mut stream);

        // Write back to the TcpStream
        stream
            .write_all(response.expect("Couldn't get response").as_bytes())
            .unwrap();
    }
}

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            handle_stream(stream.unwrap());
        });
    }

    Ok(())
}
