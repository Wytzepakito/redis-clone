use crate::config::{Config, Role, SlaveConfig};






pub fn make_array_str(vec: Vec<String>) -> String {
    let mut str = format!("*{}\r\n", vec.len());
    vec.iter().for_each(|s| str.push_str(s));
    str
}

pub fn make_bulk_str(string: String) -> String {
    format!("${}\r\n{}\r\n", string.len(), string )
}

pub fn make_info_response(config: &Config) -> String { 
    let array_string = match &config.role {
        Role::MASTER(masterConfig) => masterConfig.config_string(),
        Role::SLAVE(slaveConfig) => slaveConfig.config_string(),
    };
    array_string
}

pub fn make_simple_str(string: &str) -> String {
    format!("+{}\r\n", string)
}