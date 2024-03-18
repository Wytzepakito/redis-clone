use clap::{arg, command, value_parser, Arg};

#[derive(Debug, Clone)]
pub struct Config {
    pub role: Role,
    pub port: u32,
    pub replica_of: Option<ReplicaOf>
}

#[derive(Debug, Clone)]
pub struct ReplicaOf {
    pub host: String,
    pub port: u32,
}


impl ReplicaOf {
    pub fn new(vec: Vec<&str>) -> ReplicaOf {
        ReplicaOf {
            host: vec.get(0).expect("Couldn't get first arg of replicaof").to_string(),
            port: vec.get(1).expect("Couldn't get second arg of replicaof").parse::<u32>().expect("Couldn't parse second arg of replicaof"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Role {
    MASTER,
    SLAVE,
}

impl Config {
    pub fn new(role: Role) -> Config {
        Config { role: role, replica_of: None, port:6379 }
    }

    pub fn from_args(args: InputArgs) -> Config {
        Config {
            role: match args.replica_of {
                Some(_)=> Role::SLAVE,
                None => Role::MASTER
            },
            replica_of: args.replica_of.map( |v | 
                ReplicaOf::new(v)
            ),
            port: args.port

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
            .default_value("6379")
        )
        .arg(
            Arg::new("replicaof")
            .long("replicaof")
            .num_args(2)
        )
        .get_matches();
    let port = match matches.get_one::<u32>("port") {
        Some(port) => match port {
            x if x > &9999 => 6379 as u32,
            x => *x,
        },
        None => 6379 as u32,
    };
    let replicaof: Option<Vec<&str>> =  matches.get_many::<String>("replicaof").map(|v| v.map(|s| s.as_str()).collect());
    Config::from_args(InputArgs::new(port, replicaof))
}

pub struct InputArgs<'a> {
    port: u32,
    replica_of: Option<Vec<&'a str>>
}

impl<'a> InputArgs<'a> {
    pub fn new(port: u32, replica_of: Option<Vec<&'a str>>) -> InputArgs {
        InputArgs {
            port: port,
            replica_of: replica_of,
        }
    }
}
