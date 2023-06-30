use todo::TodoList;

use crate::todo::TodoItem;
use std::{env, error::Error};

mod todo;

#[derive(Debug)]
enum Commands {
    List,
    Check,
}

impl Commands {
    pub fn from_str(command: &str) -> Result<Self, Box<dyn Error>> {
        match command {
            "" | "l" => Ok(Self::List),
            "c" => Ok(Self::Check),
            _ => panic!("Command {} not found.", command),
        }
    }
}
fn main() {
    let mut item = TodoItem::new("I have to clean the bathroom");
    let new_title = format!("{} {}", item.title, "with some additions");
    item.title = &new_title;
    item.done = true;

    let another = TodoItem::new("Make bed.");

    let mut list = TodoList::default();
    list.add(item);
    list.add(another);

    let command_args: Vec<String> = env::args().skip(1).collect();
    let command = Commands::from_str(command_args.get(1).or(Some(&"l".to_string())).unwrap());

    eprintln!("{:?}", command);
    eprintln!("{:?}", list.show());
}
