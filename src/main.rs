use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 128];
    stream.read(&mut buffer)?;
    stream.write("+PONG\r\n".as_bytes())?;
    println!("accepted new connection");
    println!("buffer: {:?}", buffer);

    Ok(())
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
