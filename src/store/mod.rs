use std::{
    fs::{create_dir_all, OpenOptions},
    io::Read,
};

use dirs::home_dir;

use crate::todo::TodoList;

pub struct Store {
    pub todo_list: TodoList,
    pub file_name: String,
}

pub enum StoreError {
    MaximumNumberOfTodosReached,
    CannotSaveToFile,
}

impl Store {
    pub fn from_file(file_name: Option<&str>) -> Self {
        let mut folder = match home_dir() {
            Some(folder) => folder,
            None => panic!("Cannot fetch the home dir!"),
        };

        folder.push(".todo-store");

        match create_dir_all(folder) {
            Err(err) => panic!("{}", err),
            _ => (),
        };

        let file_name = match file_name {
            Some(file_name) => file_name,
            None => "store.json",
        };

        let mut file = match OpenOptions::new().read(true).create(true).open(file_name) {
            Ok(file) => file,
            Err(err) => panic!("{}", err),
        };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Err(err) => panic!("{}", err),
            _ => (),
        };

        let todo_list = match serde_json::from_str(&data) {
            Ok(todo_list) => todo_list,
            Err(err) => panic!("{}", err),
        };

        Self {
            todo_list,
            file_name: file_name.to_string(),
        }
    }

    fn save(&self) -> Result<(), StoreError> {
        let json = match serde_json::to_string(&self.todo_list) {
            Ok(json) => json,
            Err(_) => panic!("cannot convert todo list to json"),
        };

        let mut file = match OpenOptions::new()
            .read(true)
            .create(true)
            .open(&self.file_name)
        {
            Ok(file) => file,
            Err(err) => panic!("cannot open file"),
        };

        Ok(())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::from_file(None)
    }
}
