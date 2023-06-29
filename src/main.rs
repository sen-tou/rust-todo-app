use todo::TodoList;

use crate::todo::TodoItem;

mod todo;

fn main() {
    let mut item = TodoItem::new("I have to clean the bathroom");
    let new_title = format!("{} {}", item.title, "with some additions");
    item.title = &new_title;
    item.done = true;

    let another = TodoItem::new("Make bed.");

    let mut list = TodoList::default();
    list.add(item);
    list.add(another);

    eprintln!("{:?}", list.show());
}
