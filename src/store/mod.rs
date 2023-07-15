use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
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
        let mut folder = home_dir().expect("Cannot find home folder");
        folder.push(".todo-store");
        create_dir_all(&folder).expect("Cannot create config folder");

        let file_name = match file_name {
            Some(file_name) => file_name,
            None => "store.json",
        };
        let mut file_path = folder.clone();
        file_path.push(file_name);
        dbg!(&file_path);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .expect("Cannot open file");

        let mut data = String::new();
        file.read_to_string(&mut data).expect("Cannot read file");
        if data.is_empty() {
            data = "{}".to_owned();
        }

        let todo_list = serde_json::from_str(&data).expect("Cannot convert json to TodoList");

        Self {
            todo_list,
            file_name: file_name.to_string(),
        }
    }

    pub fn save(&self) -> Result<(), StoreError> {
        let json =
            serde_json::to_string(&self.todo_list).expect("Cannot convert todo list to json");

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_name)
            .expect("Cannot open file");

        dbg!(&json);

        write!(file, "{}", json).expect("Cant write to file");

        Ok(())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::from_file(None)
    }
}
