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

    let mut list = TodoList::default();
    list.add(item);
    list.add(another);

    let command_args: Vec<String> = env::args().skip(1).collect();
    if let Ok(mut command) = Commands::from_str(command_args.get(0).unwrap()) {
        command.exec(list);
    };
}
