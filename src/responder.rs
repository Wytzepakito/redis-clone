use core::fmt;
use std::{
    fmt::{format, write, Display},
    time::Duration,
};

use chrono::TimeDelta;

use crate::{config::{Config, Role}, formatter::{make_array_str, make_bulk_str, make_info_response, make_simple_str}};

pub struct Responder {}

#[derive(Debug)]
pub enum Command {
    PING,
    PONG,
    OK,
    INFO(InfoCommand),
    ECHO(String),
    SET(String, String),
    SET_EXP(String, String, TimeDelta),
    GET(String),
    REPLCONF,
}

pub enum Response { 
    PONG,
    OK,
    INFO(Config),
}

impl Response {
    pub fn respond(&self) -> String {
        match self {
            Response::PONG => make_array_str(vec![make_bulk_str(String::from("pong"))]),
            Response::OK => make_simple_str("OK"),
            Response::INFO(config) => make_info_response(&config)
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::PING => write!(f, "PING"),
            Command::PONG => write!(f, "PONG"),
            Command::OK => write!(f, "OK"),
            Command::ECHO(msg) => write!(f, "ECHO({})", msg),
            Command::SET(key, val) => write!(f, "SET({}, {})", key, val),
            Command::SET_EXP(key, val, dur) => write!(f, "SET({}, {}, {:?})", key, val, dur),
            Command::GET(key) => write!(f, "GET({})", key),
            Command::INFO(info_command) => write!(f, "INFO({})", info_command),
            Command::REPLCONF => write!(f, "REPLCONF"),
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
    pub fn new() -> Responder {
        Responder {}
    }
    pub fn ping_request(&self) -> String {
        make_array_str(vec![make_bulk_str(String::from("ping"))])
    }


    pub fn replconf_request_one(&self, replicated_port: &u32) ->  String {
        let mut veccie = vec![String::from("REPLCONF"), String::from("listening-port"), replicated_port.to_string()];    
        make_array_str(veccie.iter().map(|s| make_bulk_str(s.to_string())).collect())
    }

    pub fn replconf_request_two(&self) ->  String {
        let mut veccie = vec![String::from("REPLCONF"), String::from("capa"), String::from("psync2")];    
        make_array_str(veccie.iter().map(|s| make_bulk_str(s.to_string())).collect())
    }

}
