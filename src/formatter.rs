use crate::config::{Config, Master, Role};
use base64::{prelude::BASE64_STANDARD, Engine};

const EMPTY_RDB_BASE64: &str = "UkVESVMwMDEx+glyZWRpcy12ZXIFNy4yLjD6CnJlZGlzLWJpdHPAQPoFY3RpbWXCbQi8ZfoIdXNlZC1tZW3CsMQQAPoIYW9mLWJhc2XAAP/wbjv+wP9aog==";

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
        Role::MASTER(master_config) => master_config.config_string(),
        Role::SLAVE(slave_config) => slave_config.config_string(),
    }
}

pub fn make_fullresync_str(config: &Config) -> Vec<u8> {
    match &config.role {
        Role::MASTER(master_config) => make_resync_str(master_config),
        Role::SLAVE(_) => unimplemented!(),
    }
}

fn make_resync_str(config: &Master) -> Vec<u8> {
    let mut string = make_simple_str(format!("FULLRESYNC {} 0", &config.replication_id));

    let bytes = BASE64_STANDARD.decode(EMPTY_RDB_BASE64).unwrap();
    
    string.extend(format!("${}\r\n", bytes.len()).as_bytes());
    string.extend(bytes);
    string
}

pub fn make_simple_str(string: String) -> Vec<u8> {
    format!("+{}\r\n", string).as_bytes().to_vec()
}
