use bytes::Buf;
use chrono::TimeDelta;
use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::{model::resync_response::ResyncResponse, responder::{Command, InfoCommand, ReplConfItem}};

pub struct Marshaller {}

#[derive(Debug)]
pub enum MessageSegment {
    Array(Vec<MessageSegment>),
    SimpleString(String),
    BulkString(String),
}

impl MessageSegment {
    pub fn get_array(&self) -> Result<&Vec<MessageSegment>, String> {
        match self {
            MessageSegment::Array(v) => Ok(v),
            _ => Err(String::from("Not an array")),
        }
    }

    pub fn get_string(&self) -> Result<&str, String> {
        match self {
            MessageSegment::SimpleString(v) => Ok(v),
            MessageSegment::BulkString(v) => Ok(v),
            _ => Err(String::from("Not a string")),
        }
    }
}

impl Marshaller {
    pub fn new() -> Marshaller {
        Marshaller {}
    }
    pub fn make_command(&self, words: MessageSegment) -> Result<Command, String> {
        match words {
            MessageSegment::Array(_) => self.match_command_from_arr(words),
            MessageSegment::BulkString(_) => self.match_string(words),
            MessageSegment::SimpleString(_) => self.match_string(words),
        }
    }

    fn match_string(&self, words: MessageSegment) -> Result<Command, String> {
        // We could maybe in the future use a RegexSet for this but for now we'll use an ugly if/else block
        println!("Strings are:");
        println!("{:?}", words.get_string()?);
        if words.get_string()?.starts_with("pong") {
            Ok(Command::PONG)
        } else if words.get_string()?.starts_with("ok") {
            Ok(Command::OK)
        } else if words.get_string()?.starts_with("fullresync") {
            Ok(Command::FULLRESYNC)
        } else {
            Err(String::from("Unknown command"))
        }
    }

    fn match_command_from_arr(&self, words: MessageSegment) -> Result<Command, String> {
        let array = words.get_array()?;
        let command = array[0].get_string()?;

        match command {
            "ping" => Ok(Command::PING),
            "echo" => Ok(Command::ECHO(array[1].get_string()?.to_string())),
            "set" => self.match_set(array),
            "get" => Ok(Command::GET(array[1].get_string()?.to_string())),
            "info" => self.match_info(array),
            "replconf" => Ok(Command::REPLCONF(self.match_replconf(array)?)),
            "psync" => Ok(Command::PSYNC),
            _ => Err(String::from("Unknown command")),
        }
    }

    fn match_replconf(&self, array: &Vec<MessageSegment>) -> Result<ReplConfItem, String> {
        let replcommand = array[1].get_string()?;

        match replcommand {
            "capa" => Ok(ReplConfItem::CAPA(array[2].get_string()?.to_string())),
            "listening-port" => Ok(ReplConfItem::LISTENPORT(array[2].get_string()?.to_string())),
            _ => Err(String::from("Unknown replconf")),
        }
    }

    fn match_set(&self, array: &Vec<MessageSegment>) -> Result<Command, String> {
        match array.len() {
            3 => Ok(Command::SET(
                array[1].get_string()?.to_string(),
                array[2].get_string()?.to_string(),
            )),
            5 => Ok(Command::SETEXP(
                array[1].get_string()?.to_string(),
                array[2].get_string()?.to_string(),
                self.parse_time(array[4].get_string()?.to_string())?,
            )),
            _ => unreachable!(),
        }
    }

    fn match_info(&self, array: &Vec<MessageSegment>) -> Result<Command, String> {
        match array[1].get_string()? {
            "replication" => Ok(Command::INFO(InfoCommand::REPLICATION)),
            _ => Err(String::from("Unknown command")),
        }
    }

    fn parse_time(&self, time_string: String) -> Result<TimeDelta, String> {
        let milliseconds = time_string
            .parse::<i64>()
            .map_err(|_| String::from("Couldn't parse time_string"))?;
        TimeDelta::try_milliseconds(milliseconds)
            .ok_or(String::from("Could't parse miliseconds into TimeDelta"))
    }

    pub fn parse_resync(&self, stream: &TcpStream) -> Result<ResyncResponse, String> {

        let mut reader = BufReader::new(stream);

        let segment = self.parse_segment(&mut reader)?;

        let mut length_buffer = Vec::new();
        reader.read_until(b'\n', &mut length_buffer).map_err(|_| "Couldn't read rdb length buffer")?;
        let full: String = String::from_utf8_lossy(&length_buffer).to_string();
        let truncated: String = full[1..full.len() -2 ].to_string();
        let rdb_length = truncated.parse::<usize>().map_err(|_| "Couldn't parse rdb length string")?;


        let mut data_buffer: Vec<u8> = Vec::with_capacity(rdb_length);
        data_buffer.resize(rdb_length, 0);
        reader.read_exact(&mut data_buffer).map_err(|_| "Couldn't read rdb data buffer")?;
        let data_string: String = String::from_utf8_lossy(&data_buffer).to_string();

        Ok(ResyncResponse::new(segment, data_string))
    }

    pub fn parse_full_buffer(&self, stream: &TcpStream) -> Vec<Result<MessageSegment, String>> {

        let mut reader = BufReader::new(stream);
        let mut segment = String::new();
        let mut segments = Vec::new();

        while let Ok(line) = reader.read_line(&mut segment) {

            let (segment_type, data) = segment.trim().split_at(1);

            let one_command = match segment_type {
                "*" => self.parse_array(data, &mut reader),
                "$" => self.parse_bulk_string(&mut reader),
                "+" => self.parse_simple_string(data),
                _ => unimplemented!(),
            };
            segments.push(one_command);
        }

        segments
    }


    pub fn parse_redis_command(&self, stream: &TcpStream) -> Result<MessageSegment, String> {
        let mut reader = BufReader::new(stream);
        self.parse_segment(&mut reader)
    }


    fn parse_segment<R: BufRead>(&self, reader: &mut R) -> Result<MessageSegment, String> {
        let mut segment = String::new();
        while segment.is_empty() {
            reader
                .read_line(&mut segment)
                .map_err(|err|{
                    println!("err: {:?}", err);
                    String::from("Could not read next line")
                } )?;
        }

        let (segment_type, data) = segment.trim().split_at(1);

        match segment_type {
            "*" => self.parse_array(data, reader),
            "$" => self.parse_bulk_string(reader),
            "+" => self.parse_simple_string(data),
            _ => unimplemented!(),
        }
    }

    fn parse_array<R: BufRead>(
        &self,
        data: &str,
        reader: &mut R
    ) -> Result<MessageSegment, String> {
        let element_count: i32 = data
            .parse()
            .map_err(|e| {
                String::from(format!(
                    "Could not parse array length {data} with error {e}."
                ))
            })
            .expect("Couldn't parse this!");
        let mut words = Vec::new();

        for _ in 0..element_count {
            words.push(self.parse_segment(reader)?);
        }
        Ok(MessageSegment::Array(words))
    }

    fn parse_bulk_string<R: BufRead>(
        &self,
        reader: &mut R
    ) -> Result<MessageSegment, String> {
        let mut segment = String::new();
        reader
            .read_line(&mut segment)
            .map_err(|err| {
                println!("err: {:?}", err);
                String::from("Could not read next line")
            })?;
        Ok(MessageSegment::BulkString(
            segment.trim().to_owned().to_ascii_lowercase(),
        ))
    }

    fn parse_simple_string(&self, data: &str) -> Result<MessageSegment, String> {
        Ok(MessageSegment::SimpleString(
            data.to_string().to_ascii_lowercase(),
        ))
    }
}
