#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    id: i32,
    text: String,
    done: bool,
}

impl Todo {
    pub fn new(id: i32, text: String, done: bool) -> Self {
        Self { id, text, done }
    }
}