use crate::DECIMAL_RADIX;

pub struct Responder {}


pub enum Command {
    PING,
    ECHO(String),
}

impl Responder {
    pub fn make_response(&self, command: Command) -> Result<String, String> {
        match(command) {
            Command::PING => Ok(format!("+PONG]\r\n")),
            Command::ECHO(msg) => Ok(format!("${}\r\n{}\r\n", msg.len(), msg)),
            _ => Err(String::from("Unknown command")),
        }

    }
}
