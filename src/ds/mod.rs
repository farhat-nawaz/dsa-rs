mod linked_list;
mod stack;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug, PartialEq)]
enum DSError {
    LinkedListError,
    StackEmpty,
}
