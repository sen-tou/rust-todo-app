use std::error::Error;

use crate::todo::TodoList;

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
            LIST => Ok(Self::List),
            CHECK => Ok(Self::Check),
            _ => panic!("Command {} not found.", command),
        }
    }

    pub fn exec(&mut self, todo_list: TodoList) {
        match *self {
            Self::List => self.list(todo_list),
            Self::Check => self.check(),
        }
    }

    fn list(&self, todo_list: TodoList) {
        for item in todo_list.as_vec() {
            println!("{}", item);
        }
    }

    fn check(&mut self) {
        todo!()
    }
}
