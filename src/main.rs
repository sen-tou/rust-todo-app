use std::env;

use todo::TodoList;

use crate::command::Commands;
use crate::todo::TodoItem;

mod command;
mod todo;

fn main() {
    let mut item = TodoItem::new("I have to clean the bathroom");
    let new_title = format!("{} {}", item.title, "with some additions");
    item.title = &new_title;
    item.done = true;

    let another = TodoItem::new("Make bed.");

    let mut todo_list = TodoList::default();
    todo_list.add(item);
    todo_list.add(another);

    let mut command = Commands::new(todo_list);
    command.exec();
}
