use redis_starter_rust::RedisServer;





fn main() -> std::io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let mut server = RedisServer::new();
    server.run();

    Ok(())
}
