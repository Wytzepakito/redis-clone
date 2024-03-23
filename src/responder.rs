use core::fmt;
use std::{
    fmt::{format, write, Display},
    time::Duration,
};

use chrono::TimeDelta;

use crate::{config::{Config, Role}, formatter::{make_array_str, make_bulk_str, make_info_str, make_simple_str}};

pub struct Responder {}

#[derive(Debug, PartialEq)]
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
    PSYNC,
    FULLRESYNC,
}

pub enum Response { 
    PONG,
    OK,
    INFO(Config),
}

impl Response {
    pub fn respond(&self) -> Vec<u8> {
        match self {
            Response::PONG => make_array_str(vec![make_bulk_str(String::from("pong"))]),
            Response::OK => make_simple_str(String::from("OK")),
            Response::INFO(config) => make_info_str(&config)
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
            Command::PSYNC => write!(f, "PSYNC"),
            Command::FULLRESYNC => write!(f, "FULLRESYNC"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
    pub fn ping_request(&self) -> Vec<u8> {
        make_array_str(vec![make_bulk_str(String::from("ping"))])
    }

    pub fn pong_response(&self) -> Vec<u8> {
       make_simple_str(String::from("PONG")) 
    }

    pub fn empty_response(&self) -> Vec<u8> {
        String::from("").as_bytes().to_vec()
    }

    pub fn ok_reponse(&self) -> Vec<u8> {
        make_simple_str(String::from("OK"))
    }

    pub fn empty_get_reponse(&self) -> Vec<u8> {
        String::from("$-1\r\n").as_bytes().to_vec()
    }

    pub fn replconf_request_one(&self, replicated_port: &u32) ->  Vec<u8> {
        let mut veccie = vec![String::from("REPLCONF"), String::from("listening-port"), replicated_port.to_string()];    
        make_array_str(veccie.iter().map(|s| make_bulk_str(s.to_string())).collect())
    }

    pub fn replconf_request_two(&self) ->  Vec<u8> {
        let mut veccie = vec![String::from("REPLCONF"), String::from("capa"), String::from("psync2")];    
        make_array_str(veccie.iter().map(|s| make_bulk_str(s.to_string())).collect())
    }

    pub fn psync_request(&self) ->  Vec<u8> {
        let mut veccie = vec![String::from("PSYNC"), String::from("?"), String::from("-1")];    
        make_array_str(veccie.iter().map(|s| make_bulk_str(s.to_string())).collect())
    }
}
