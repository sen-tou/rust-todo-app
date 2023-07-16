use anyhow::Result;
use store::Store;

use crate::command::Commands;

mod command;
mod error;
mod store;
mod todo;

fn main() -> Result<()> {
    let mut store = Store::from_file(None)?;
    let todo_list = &mut store.todo_list;
    let mut command = Commands::new(todo_list);

    command.exec()?;
    store.save()
}
