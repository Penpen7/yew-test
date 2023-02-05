#[derive(Clone, PartialEq, Debug)]
pub struct TodoModel {
    pub title: String,
    pub has_done: bool,
}

impl TodoModel {
    pub fn new(title: String) -> TodoModel {
        TodoModel {
            title,
            has_done: false,
        }
    }
}

pub enum Message {
    New(TodoModel),
    Done(usize),
    Doing(usize),
}
