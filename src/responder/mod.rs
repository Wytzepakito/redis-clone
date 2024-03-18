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

        let array_string = match &config.role {
            Role::MASTER(masterConfig) => masterConfig.config_string(),
            Role::SLAVE(_) => String::from("$10\r\nrole:slave\r\n"),
        };
        println!("{}", array_string);
        array_string
    }

    fn make_array_str(&self,vec: Vec<String>) -> String {
        let mut str = format!("*{}\r\n", vec.len());
        vec.iter().for_each(|s| str.push_str(s));
        str
    }
}
