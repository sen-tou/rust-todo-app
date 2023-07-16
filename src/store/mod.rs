use anyhow::Result;
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use dirs::home_dir;

use crate::{error::TodoError, todo::TodoList};

pub struct Store {
    pub todo_list: TodoList,
    pub location: PathBuf,
}

impl Store {
    pub fn from_file(file_name: Option<&str>) -> Result<Self> {
        let mut folder = home_dir().expect("Cannot open file");
        folder.push(".todo-store");
        create_dir_all(&folder).map_err(|err| TodoError::StoreFileError(err.to_string()))?;

        let file_name = match file_name {
            Some(file_name) => file_name,
            None => "store.json",
        };
        let mut location = folder.clone();
        location.push(file_name);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&location)
            .map_err(|err| TodoError::StoreFileError(err.to_string()))?;

        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|err| TodoError::StoreFileError(err.to_string()))?;

        if data.trim().is_empty() {
            data = "{}".to_owned();
        }

        let todo_list = serde_json::from_str(&data).expect("Cannot convert json to TodoList");

        Ok(Self {
            todo_list,
            location,
        })
    }

    pub fn save(&self) -> Result<()> {
        let json =
            serde_json::to_string(&self.todo_list).expect("Cannot convert todo list to json");

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.location)
            .expect("Cannot open file");

        write!(file, "{}", json).expect("Cant write to file");

        Ok(())
    }
}
