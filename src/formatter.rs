use crate::config::{Config, Master, Role, Slave};
use base64::{prelude::BASE64_STANDARD, Engine};

const empty_rdb_base64: &str = "UkVESVMwMDEx+glyZWRpcy12ZXIFNy4yLjD6CnJlZGlzLWJpdHPAQPoFY3RpbWXCbQi8ZfoIdXNlZC1tZW3CsMQQAPoIYW9mLWJhc2XAAP/wbjv+wP9aog==";

pub fn make_array_str(vec: Vec<Vec<u8>>) -> Vec<u8> {
    let mut str = format!("*{}\r\n", vec.len()).as_bytes().to_vec();
    vec.iter().for_each(|s| str.extend(s));
    str
}

pub fn make_bulk_str(string: String) -> Vec<u8> {
    format!("${}\r\n{}\r\n", string.len(), string)
        .as_bytes()
        .to_vec()
}

pub fn make_rdb_file_str(string: String) -> Vec<u8> {
    format!("${}\r\n{}", string.len(), string)
        .as_bytes()
        .to_vec()
}

pub fn make_info_str(config: &Config) -> Vec<u8> {
    match &config.role {
        Role::MASTER(masterConfig) => masterConfig.config_string(),
        Role::SLAVE(slaveConfig) => slaveConfig.config_string(),
    }
}

pub fn make_fullresync_str(config: &Config) -> Vec<u8> {
    match &config.role {
        Role::MASTER(masterConfig) => make_resync_str(masterConfig),
        Role::SLAVE(slaveConfig) => unimplemented!(),
    }
}

fn make_resync_str(config: &Master) -> Vec<u8> {
    let mut string = make_simple_str(format!("FULLRESYNC {} 0", &config.replication_id));

    let bytes = BASE64_STANDARD.decode(empty_rdb_base64).unwrap();
    string.extend(format!("${}\r\n", bytes.len()).as_bytes());
    string.extend(bytes);
    string
}

pub fn make_simple_str(string: String) -> Vec<u8> {
    format!("+{}\r\n", string).as_bytes().to_vec()
}
