#[derive(Debug)]
struct TodoItem<'a> {
    title: &'a str,
}

impl<'a> TodoItem<'a> {
    fn new(title: &'a str) -> Self {
        Self { title }
    }

    fn edit(&mut self, title: &'a str) {
        self.title = title;
    }
}

fn main() {
    let item = TodoItem::new("I have to clean the bathroom");
    eprintln!("{:?}", item);
}
