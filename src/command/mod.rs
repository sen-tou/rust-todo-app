use std::env::{self};

use crate::todo::{TodoItem, TodoList};

pub const LIST: &str = "l";
pub const CHECK: &str = "c";
pub const ADD: &str = "a";

#[derive(Debug)]
pub struct Commands<'a> {
    command_args: Vec<String>,
    todo_list: &'a mut TodoList,
}

impl<'a> Commands<'a> {
    pub fn new(todo_list: &'a mut TodoList) -> Self {
        Self {
            command_args: env::args().collect(),
            todo_list,
        }
    }

    pub fn exec(&mut self) {
        match self.command_args.get(1).map(|s| s.as_str()) {
            Some(LIST) => self.list(),
            Some(CHECK) => self.check(),
            Some(ADD) => self.add(),
            _ => todo!(),
        }
    }

    fn list(&self) {
        for item in self.todo_list.as_vec() {
            println!("{}", item);
        }
    }

    fn check(&mut self) {
        todo!()
    }

    fn add(&mut self) {
        let joined = self.command_args[1..].join(" ");
        let item = TodoItem::new(&joined);
        self.todo_list.add(item)
    }
}
