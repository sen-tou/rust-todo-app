use std::env::{self, Args};

use crate::todo::{TodoItem, TodoList};

pub const LIST: &str = "l";
pub const CHECK: &str = "c";
pub const ADD: &str = "a";

#[derive(Debug)]
pub struct Commands<'a> {
    command_args: Args,
    todo_list: &'a mut TodoList,
}

impl<'a> Commands<'a> {
    pub fn new(todo_list: &'a mut TodoList) -> Self {
        Self {
            command_args: env::args(),
            todo_list,
        }
    }

    pub fn exec(&mut self) {
        match self.command_args.nth(1).as_deref() {
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
        let item = TodoItem::new(&self.command_args.nth(2).expect("Title not provided."));
        self.todo_list.add(item)
    }
}
