use std::ops::Deref;

#[derive(Debug)]
pub struct TodoItem<'a> {
    pub title: &'a str,
    pub done: bool,
}

impl<'a> TodoItem<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title, done: false }
    }
}

// It's reasonable to not have this list generic because we can assume that
// a todo list always contains todo items
// We could implement a trait say Completable that abstracts items that can be
// completed just like todo items. This way we could get rid of the lifetime
// I will not however to be able to work with lifetimes is this example
#[derive(Default, Debug)]
pub struct TodoList<'a> {
    collection: Vec<TodoItem<'a>>,
}

impl<'a> TodoList<'a> {
    pub fn add(&mut self, item: TodoItem<'a>) -> () {
        self.collection.push(item);
    }
}

impl<'a> Deref for TodoList<'a> {
    type Target = Vec<TodoItem<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}