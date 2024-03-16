use core::fmt;
use std::fmt::Display;

pub struct Responder {}

#[derive(Debug)]
pub enum Command {
    PING,
    ECHO(String),
    SET(String, String),
    GET(String),
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::PING => write!(f, "PING"),
            Command::ECHO(msg) => write!(f, "ECHO({})", msg),
            Command::SET(key, val) => write!(f, "SET({}, {})", key, val),
            Command::GET(key) => write!(f, "GET({})", key),
        }
    }
}

impl Responder {}
