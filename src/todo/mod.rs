use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::error::TodoError;

const MAX_NUMBER_OF_TODOS: usize = 10;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub title: String,
    pub done: bool,
}

impl TodoItem {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            done: false,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TodoList {
    #[serde(default)]
    collection: Vec<TodoItem>,
}

impl TodoList {
    pub fn add(&mut self, item: TodoItem) -> Result<()> {
        if self.collection.len() == MAX_NUMBER_OF_TODOS {
            Err(TodoError::MaximumNumberOfTodosReached(MAX_NUMBER_OF_TODOS))?
        }

        self.collection.push(item);

        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Result<()> {
        match self.collection.get(index) {
            Some(_) => {
                self.collection.remove(index);
                Ok(())
            }
            None => Err(TodoError::TodoItemNotFound(index))?,
        }
    }

    pub fn check(&mut self, index: usize) -> Result<()> {
        match self.collection.get_mut(index) {
            Some(item) => {
                let toggle = !item.done;
                item.done = toggle;
                Ok(())
            }
            None => Err(TodoError::TodoItemNotFound(index))?,
        }
    }

    pub fn formatted_list(&self) -> Vec<String> {
        self.collection
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let x = if item.done { "x" } else { " " };

                format!("{}: [{}] {}", index, x, item.title)
            })
            .collect()
    }
}
