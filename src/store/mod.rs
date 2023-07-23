use anyhow::Result;
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

use dirs::home_dir;

use crate::todo::TodoList;

pub struct Store {
    pub todo_list: TodoList,
    pub location: PathBuf,
}

impl Store {
    pub fn from_file(file_name: Option<&str>) -> Result<Self> {
        let mut folder = home_dir().expect("Cannot open file");
        folder.push(".todo-store");
        create_dir_all(&folder)?;

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
            .open(&location)?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        data = data.trim().to_owned();
        if data.trim().is_empty() {
            data = "{}".to_owned();
        }
        dbg!(&data);

        let todo_list = serde_json::from_str(&data)?;

        Ok(Self {
            todo_list,
            location,
        })
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string(&self.todo_list)?;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.location)?;

        write!(file, "{}", json)?;

        Ok(())
    }
}
