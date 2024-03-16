use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread;

use redis_starter_rust::Redis;

fn handle_stream(mut stream: TcpStream, num: usize) {
    loop {
        // Process the received data (you can replace this with your own logic)
        let mut redis = Redis::new();
        let mut words = redis.marshaller.parse_redis_command(&mut stream);
        let command = redis.make_response(words.expect("Couldn't get words"));

        let response = redis.responder.make_response(command.expect("Couldn't get command"));

        // Write back to the TcpStream
        stream.write_all(response.expect("Couldn't get response").as_bytes()).unwrap();
    }
}

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for (i, stream) in listener.incoming().enumerate() {
        thread::spawn(move || {
            handle_stream(stream.unwrap(), i);
        });
    }

    Ok(())
}
