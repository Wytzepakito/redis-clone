use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};


const MESSAGE_SIZE: usize = 5;

fn handle_client(mut stream: TcpStream)  {

     let mut buffer = [0; 1024]; // Buffer to store received data

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
                println!("Received: {:?}", &string_data);

                // Write back to the TcpStream
                stream.write_all(b"+PONG\r\n").unwrap();
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
    for stream in listener.incoming() {
        handle_client(stream?);
    }


    Ok(())
}
