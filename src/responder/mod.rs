use core::fmt;
use std::{fmt::Display, time::Duration};

use chrono::TimeDelta;

pub struct Responder {}

#[derive(Debug)]
pub enum Command {
    PING,
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
        }
    }
}

impl Responder {}
