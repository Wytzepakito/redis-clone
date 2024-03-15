use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread;

use redis_starter_rust::{make_response, parse_redis_command, MAX_SIZE};



fn handle_client(mut stream: TcpStream, num: usize) {
    let mut buffer = [0; MAX_SIZE]; // Buffer to store received data

    loop {
        // Read data from the TcpStream
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed
                    break;
                }

                // Process the received data (you can replace this with your own logic)
                let received_data = &buffer[..bytes_read];
                let string_data = String::from_utf8(received_data.to_vec()).unwrap();
                println!("Received: {:?} on thread {}", &string_data, num);
                let mut words = parse_redis_command(&buffer);
                let response = make_response(words);
                // Write back to the TcpStream
                println!("Writing back: {:?}", String::from_utf8(response.clone()).unwrap());
                stream.write_all(&response.into_boxed_slice()).unwrap();
            }
            Err(err) => {
                eprintln!("Error reading from TcpStream: {}", err);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for (i, stream) in listener.incoming().enumerate() {
        thread::spawn(move || {
            handle_client(stream.unwrap(), i);
        });
    }

    Ok(())
}
