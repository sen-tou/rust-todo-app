#[derive(Debug)]
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

// It's reasonable to not have this list generic because we can assume that
// a todo list always contains todo items
// We could implement a trait say Completable that abstracts items that can be
// completed just like todo items. This way we could get rid of the lifetime
// I will not however to be able to work with lifetimes is this example
#[derive(Default, Debug)]
pub struct TodoList {
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
