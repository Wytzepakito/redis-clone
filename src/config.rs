use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use clap::{arg, command, value_parser, Arg};

use crate::formatter::{make_array_str, make_bulk_str};

#[derive(Debug, Clone)]
pub struct Config {
    pub role: Role,
    pub port: u32,
}

#[derive(Debug, Clone)]
pub enum Role {
    MASTER(Master),
    SLAVE(Slave),
}

impl Role {
    pub fn get_slave(&self) -> Result<&Slave, String> {
        match self {
            Role::SLAVE(v) => Ok(v),
            _ => Err(String::from("Not a slave")),
        }
    }
}

impl Config {
    pub fn new(role: Role) -> Config {
        Config {
            role: role,
            port: 6379,
        }
    }

    pub fn from_args(args: InputArgs) -> Config {
        Config {
            role: match args.replica_of {
                Some(args) => Role::SLAVE(Slave::new(args)),
                None => Role::MASTER(Master::new()),
            },
            port: args.port,
        }
    }

    pub fn get_master(&self) -> &Master {
        match &self.role {
            Role::MASTER(master_config) => master_config,
            Role::SLAVE(_) => panic!("This can't happen"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Slave {
    pub replicated_host: String,
    pub replicated_port: u32,
}

impl Slave {
    pub fn new(vec: Vec<&str>) -> Slave {
        Slave {
            replicated_host: vec
                .get(0)
                .expect("Couldn't get first arg of replicaof")
                .to_string(),
            replicated_port: vec
                .get(1)
                .expect("Couldn't get second arg of replicaof")
                .parse::<u32>()
                .expect("Couldn't parse second arg of replicaof"),
        }
    }

    pub fn config_string(&self) -> Vec<u8> {
        make_bulk_str(self.role_string())
    }

    fn role_string(&self) -> String {
        String::from("role:slave")
    }
}

#[derive(Debug, Clone)]
pub struct Master {
    pub replication_id: String,
    pub offset: u32,
    pub streams: Arc<Mutex<Vec<TcpStream>>>,
}

impl Master {
    pub fn new() -> Master {
        Master {
            replication_id: String::from("8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb"),
            offset: 0,
            streams: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn config_string(&self) -> Vec<u8> {
        let strings = vec![
            self.role_string(),
            self.replication_id_out(),
            self.replication_offset_out(),
        ];
        make_bulk_str(strings.join("\n"))
    }

    pub fn role_string(&self) -> String {
        String::from("role:master")
    }
    pub fn replication_id_out(&self) -> String {
        format!("master_replid:{}", self.replication_id)
    }

    pub fn replication_offset_out(&self) -> String {
        format!("master_repl_offset:{}", self.offset)
    }

    pub fn propagate_commands(&self, command: Vec<u8>) {
        let locked_streams = self.streams.lock().unwrap();
        for mut stream in locked_streams.iter() {
            stream
                .write(&command)
                .expect("Error when writing to slave stream");
        }
    }
}

pub struct InputArgs<'a> {
    port: u32,
    replica_of: Option<Vec<&'a str>>,
}

impl<'a> InputArgs<'a> {
    pub fn new(port: u32, replica_of: Option<Vec<&'a str>>) -> InputArgs {
        InputArgs {
            port: port,
            replica_of: replica_of,
        }
    }
}

pub fn get_config() -> Config {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -p --port <int> "Sets a custom port number"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(u32))
            .default_value("6379"),
        )
        .arg(Arg::new("replicaof").long("replicaof").num_args(2))
        .get_matches();
    let port = match matches.get_one::<u32>("port") {
        Some(port) => match port {
            x if x > &9999 => 6379 as u32,
            x => *x,
        },
        None => 6379 as u32,
    };
    let replicaof: Option<Vec<&str>> = matches
        .get_many::<String>("replicaof")
        .map(|v| v.map(|s| s.as_str()).collect());
    Config::from_args(InputArgs::new(port, replicaof))
}
