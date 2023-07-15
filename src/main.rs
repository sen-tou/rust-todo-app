use store::Store;

use crate::command::Commands;

mod command;
mod store;
mod todo;

fn main() {
    let mut store = Store::default();
    let todo_list = &mut store.document;
    let mut command = Commands::new(todo_list);
    command.exec();
}
