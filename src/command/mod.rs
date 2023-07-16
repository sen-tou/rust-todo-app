use crate::{
    error::TodoError,
    todo::{TodoItem, TodoList},
};
use anyhow::Result;

use std::{
    env::{self},
    num::ParseIntError,
};

pub const HELP: &str = "h";
pub const LIST: &str = "l";
pub const CHECK: &str = "c";
pub const ADD: &str = "a";
pub const REMOVE: &str = "r";

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

    pub fn exec(&mut self) -> Result<()> {
        match self.command_args.get(1).map(|s| s.as_str()) {
            Some(HELP) => self.help(),
            Some(LIST) => self.list(),
            Some(CHECK) => self.check(),
            Some(ADD) => self.add(),
            Some(REMOVE) => self.remove(),
            Some(unknown) => Err(TodoError::NoCommand(unknown.to_owned()))?,
            None => Err(TodoError::NoCommand("None".to_owned()))?,
        }
    }

    fn help(&self) -> Result<()> {
        print!(
            r#"
todo command help:
    commands:
    - h: help about this command
    - l: list all todos
    - c: check a todo when ready
    - a: add a todo to the list 
    - r: remove a todo from the list
"#
        );

        Ok(())
    }

    fn list(&self) -> Result<()> {
        for item in self.todo_list.formatted_list() {
            println!("{}", item);
        }

        Ok(())
    }

    fn remove(&mut self) -> Result<()> {
        let index_str = self
            .command_args
            .get(2)
            .ok_or_else(|| TodoError::MissingCommandArgs)?;

        let index = index_str
            .parse()
            .map_err(|err: ParseIntError| TodoError::Other(err.to_string()))?;

        self.todo_list.remove(index)
    }

    fn check(&mut self) -> Result<()> {
        let index_str = self
            .command_args
            .get(2)
            .ok_or_else(|| TodoError::MissingCommandArgs)?;

        let index = index_str
            .parse()
            .map_err(|err: ParseIntError| TodoError::Other(err.to_string()))?;

        self.todo_list.check(index)
    }

    fn add(&mut self) -> Result<()> {
        let joined = self.command_args[2..].join(" ");
        let item = TodoItem::new(&joined);

        self.todo_list.add(item)
    }
}
