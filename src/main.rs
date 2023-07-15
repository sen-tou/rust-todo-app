use store::Store;

use crate::command::Commands;

mod command;
mod store;
mod todo;

fn main() {
    let mut store = Store::default();
    let todo_list = &mut store.todo_list;
    let mut command = Commands::new(todo_list);
    command.exec();
    let _ = store.save();
}
