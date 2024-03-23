use crate::config::{Config, Role, SlaveConfig};






pub fn make_array_str(vec: Vec<String>) -> String {
    let mut str = format!("*{}\r\n", vec.len());
    vec.iter().for_each(|s| str.push_str(s));
    str
}

pub fn make_bulk_str(string: String) -> String {
    format!("${}\r\n{}\r\n", string.len(), string )
}

pub fn make_info_str(config: &Config) -> String { 
    match &config.role {
        Role::MASTER(masterConfig) => masterConfig.config_string(),
        Role::SLAVE(slaveConfig) => slaveConfig.config_string(),
    }
}

pub fn make_fullresync_str(config: &Config) -> String { 
    match &config.role {
        Role::MASTER(masterConfig) => make_simple_str(format!("FULLRESYNC {} 0", &masterConfig.replication_id).as_str()),
        Role::SLAVE(slaveConfig) => unimplemented!()
    }
}

pub fn make_simple_str(string: &str) -> String {
    format!("+{}\r\n", string)
}