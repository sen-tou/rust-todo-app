use crate::todo::TodoList;

pub const LIST: &str = "l";
pub const CHECK: &str = "c";
pub const ADD: &str = "a";

#[derive(Debug)]
pub struct Commands<'a> {
    todo_list: TodoList<'a>,
}

impl<'a> Commands<'a> {
    pub fn new(mut todo_list: TodoList<'a>) -> Self {
        Self { todo_list }
    }

    pub fn exec(&mut self, command: &str) {
        match command {
            LIST => self.list(),
            CHECK => self.check(),
            ADD => self.add(),
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

    fn add(&mut self) {}
}
