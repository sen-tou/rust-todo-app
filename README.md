# Rust todo app

Yes, just what you think it is. A simple todo CLI app to make myself familiar
with the rust lang.

## Features

- [x] add/remove/edit todo item
- [x] show list of todos
- [x] tick them when finished
- [ ] interactive mode?
- [x] persist and read them from a json file
- [ ] write some tests
- [x] some error handling
- [ ] show help text
  - [x] overview
  - [ ] examples for all

## Running

```bash
cargo build --release

# show help
./target/release/rust-todo-app h

# show list of todos (default storage is ~/.todo-store/store.json)
./target/release/rust-todo-app l

# add a todo to the list
./target/release/rust-todo-app a A new Todo
./target/release/rust-todo-app a "Another one"

# edit a todo (index)
./target/release/rust-todo-app e 3 "This is the title I replace the original one with"

# remove a todo (index)
./target/release/rust-todo-app r 1

# check/mark as done (index)
./target/release/rust-todo-app c 1
```
