use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub struct Marshaller {}

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
        match (self) {
            MessageSegment::SimpleString(v) => Ok(v),
            MessageSegment::BulkString(v) => Ok(v),
            _ => Err(String::from("Not a string")),
        }
    }
}

impl Marshaller {
    pub fn parse_redis_command(
        &self,
        stream: &mut std::net::TcpStream,
    ) -> Result<MessageSegment, String> {
        let mut reader = BufReader::new(stream);

        self.parse_segment(&mut reader)
    }
    pub fn parse_segment(
        &self,
        reader: &mut BufReader<&mut TcpStream>,
    ) -> Result<MessageSegment, String> {
        let mut segment = String::new();
        while segment.is_empty() {
            reader
                .read_line(&mut segment)
                .map_err(|_| String::from("Could not read next line"))?;
        }
        let (segment_type, data) = segment.trim().split_at(1);

        match segment_type {
            "*" => self.parse_array(data, reader),
            "$" => self.parse_bulk_string(data, reader),
            "+" => self.parse_simple_string(data, reader),
            _ => unimplemented!(),
        }
    }

    fn parse_array(
        &self,
        data: &str,
        reader: &mut BufReader<&mut TcpStream>,
    ) -> Result<MessageSegment, String> {
        let element_count: i32 = data
            .parse()
            .map_err(|e| {
                String::from(format!(
                    "Could not parse array length {data} with error {e}."
                ))
            })
            .expect("Couldn't get element count");
        let mut words = Vec::new();

        for _ in 0..element_count {
            words.push(self.parse_segment(reader)?);
        }
        Ok(MessageSegment::Array(words))
    }

    pub fn parse_bulk_string(
        &self,
        data: &str,
        reader: &mut BufReader<&mut TcpStream>,
    ) -> Result<MessageSegment, String> {
        let mut segment = String::new();
        reader
            .read_line(&mut segment)
            .map_err(|_| String::from("Could not read next line"))?;
        Ok(MessageSegment::BulkString(
            segment.trim().to_owned().to_ascii_lowercase(),
        ))
    }

    pub fn parse_simple_string(
        &self,
        data: &str,
        reader: &mut BufReader<&mut TcpStream>,
    ) -> Result<MessageSegment, String> {
        Ok(MessageSegment::SimpleString(data.to_string()))
    }
}
