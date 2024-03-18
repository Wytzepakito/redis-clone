pub struct Config {
    pub role: Role,
}

pub enum Role {
    MASTER,
    SLAVE,
}

impl Config {
    pub fn new(role: Role) -> Config {
        Config { role: role }
    }
}
