use std::error::Error;

pub const LIST: &str = "l";
pub const CHECK: &str = "c";

#[derive(Debug)]
pub enum Commands {
    List,
    Check,
}

impl Commands {
    pub fn from_str(command: &str) -> Result<Self, Box<dyn Error>> {
        match command {
            "" | LIST => Ok(Self::List),
            CHECK => Ok(Self::Check),
            _ => panic!("Command {} not found.", command),
        }
    }
}
