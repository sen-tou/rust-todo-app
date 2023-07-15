use serde::Deserialize;
use serde::Serialize;

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
    pub fn add(&mut self, item: TodoItem) -> () {
        self.collection.push(item);
    }

    pub fn as_vec(&self) -> Vec<String> {
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
