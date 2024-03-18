use core::fmt;
use std::{
    fmt::{write, Display},
    time::Duration,
};

use chrono::TimeDelta;

use crate::config::{Config, Role};

pub struct Responder {}

#[derive(Debug)]
pub enum Command {
    PING,
    INFO(InfoCommand),
    ECHO(String),
    SET(String, String),
    SET_EXP(String, String, TimeDelta),
    GET(String),
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::PING => write!(f, "PING"),
            Command::ECHO(msg) => write!(f, "ECHO({})", msg),
            Command::SET(key, val) => write!(f, "SET({}, {})", key, val),
            Command::SET_EXP(key, val, dur) => write!(f, "SET({}, {}, {:?})", key, val, dur),
            Command::GET(key) => write!(f, "GET({})", key),
            Command::INFO(info_command) => write!(f, "INFO({})", info_command),
        }
    }
}

#[derive(Debug)]
pub enum InfoCommand {
    REPLICATION,
}

impl Display for InfoCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfoCommand::REPLICATION => write!(f, "REPLICATION"),
        }
    }
}

impl Responder {
    pub fn info_response(&self, config: &Config) -> String {
        let mut info_string = String::new();
        //info_string.push_str("# replication");
        match config.role {
            Role::MASTER => info_string.push_str("$11\r\nrole:master\r\n"),
            Role::SLAVE => info_string.push_str("$10\r\nrole:slave\r\n"),
        }

        info_string
    }
}
