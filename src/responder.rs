use core::fmt;
use std::{fmt::Display, io::Write, net::TcpStream};

use chrono::TimeDelta;

use crate::{
    config::Config,
    formatter::{make_array_str, make_bulk_str, make_info_str, make_simple_str},
};

pub struct Responder {}

#[derive(Debug, PartialEq)]
pub enum Command {
    PING,
    PONG,
    OK,
    INFO(InfoCommand),
    ECHO(String),
    SET(String, String),
    SETEXP(String, String, TimeDelta),
    GET(String),
    REPLCONF(ReplConfItem),
    PSYNC,
    FULLRESYNC,
}

#[derive(Debug, PartialEq)]
pub enum ReplConfItem {
    LISTENPORT(String),
    CAPA(String),
}

impl Display for ReplConfItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReplConfItem::CAPA(capability) => write!(f, "CAPA({})", capability),
            ReplConfItem::LISTENPORT(port) => write!(f, "LISTENPORT({})", port),
        }
    }
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
            Response::INFO(config) => make_info_str(&config),
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
            Command::SETEXP(key, val, dur) => write!(f, "SET({}, {}, {:?})", key, val, dur),
            Command::GET(key) => write!(f, "GET({})", key),
            Command::INFO(info_command) => write!(f, "INFO({})", info_command),
            Command::REPLCONF(replconf_item) => write!(f, "REPLCONF({})", replconf_item),
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

    pub fn make_reponse(&self) -> Vec<u8> {
        unimplemented!()
    }

    pub fn copy_request(&self, command: &Command) -> Vec<u8> {
        match command {
            Command::OK => make_simple_str(String::from("OK")),
            Command::ECHO(echo_words) => make_array_str(vec![
                make_bulk_str(String::from("echo")),
                make_bulk_str(echo_words.to_string()),
            ]),
            Command::GET(get_words) => make_array_str(vec![
                make_bulk_str(String::from("get")),
                make_bulk_str(get_words.to_string()),
            ]),
            Command::SET(key, val) => make_array_str(vec![
                make_bulk_str(String::from("set")),
                make_bulk_str(key.to_string()),
                make_bulk_str(val.to_string()),
            ]),
            Command::SETEXP(key, val, time) => make_array_str(vec![
                make_bulk_str(String::from("set")),
                make_bulk_str(key.to_string()),
                make_bulk_str(val.to_string()),
                make_bulk_str(String::from("px")),
                make_bulk_str(time.to_string()),
            ]),
            _ => unimplemented!("These commands were not implemented yet"),
        }
    }

    pub fn send_ping_request(&self, mut stream: &TcpStream) {
        stream.write_all(&make_array_str(vec![make_bulk_str(String::from("ping"))])).unwrap();
    }

    pub fn send_pong_response(&self, mut stream: &TcpStream) {
        stream.write_all(&make_simple_str(String::from("PONG"))).unwrap();
    }

    pub fn send_empty_response(&self, mut stream: &TcpStream) {
        stream.write_all(&String::from("").as_bytes().to_vec()).unwrap();
    }

    pub fn send_ok_reponse(&self, mut stream: &TcpStream) {
        stream.write_all(&make_simple_str(String::from("OK"))).unwrap();
    }

    pub fn send_empty_get_reponse(&self, mut stream: &TcpStream) {
        stream.write_all(&String::from("$-1\r\n").as_bytes().to_vec()).unwrap();
    }

    pub fn send_string_response(&self, mut stream: &TcpStream, message: String) {
        stream.write_all(&make_bulk_str(message)).unwrap();
    }

    pub fn send_vec_response(&self, mut stream: &TcpStream, info_string: Vec<u8>) {
        stream.write_all(&info_string).unwrap();
    }

    pub fn send_rdb(&self, mut stream: &TcpStream) {
        
    }

    pub fn replconf_request_one(&self, replicated_port: &u32) -> Vec<u8> {
        let veccie = vec![
            String::from("REPLCONF"),
            String::from("listening-port"),
            replicated_port.to_string(),
        ];
        make_array_str(
            veccie
                .iter()
                .map(|s| make_bulk_str(s.to_string()))
                .collect(),
        )
    }

    pub fn replconf_request_two(&self) -> Vec<u8> {
        let veccie = vec![
            String::from("REPLCONF"),
            String::from("capa"),
            String::from("psync2"),
        ];
        make_array_str(
            veccie
                .iter()
                .map(|s| make_bulk_str(s.to_string()))
                .collect(),
        )
    }

    pub fn psync_request(&self) -> Vec<u8> {
        let veccie = vec![String::from("PSYNC"), String::from("?"), String::from("-1")];
        make_array_str(
            veccie
                .iter()
                .map(|s| make_bulk_str(s.to_string()))
                .collect(),
        )
    }
}
