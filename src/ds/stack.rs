use super::{DSError, Node};

struct Stack<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let head = self.head.take();

        let mut node = Node::new(value);
        node.next = head;

        self.head = Some(Box::new(node));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Result<T, DSError> {
        let Some(mut head) = self.head.take() else {
            return Err(DSError::StackEmpty);
        };

        self.head = head.next;
        head.next = None;
        self.length -= 1;

        Ok(head.value)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) {
        let _ = self.head.take();
    }
}

mod tests {
    use crate::ds::DSError;

    use super::{Node, Stack};

    #[test]
    fn create_stack() {
        let stack: Stack<u32> = Stack::new();

        assert!(stack.head.is_none());
        assert_eq!(stack.length, 0);
    }

    #[test]
    fn push() {
        let mut ll: Stack<i32> = Stack::new();

        ll.push(5);

        assert!(ll.head.is_some());
        assert_eq!(ll.len(), 1);

        let head = ll.head.unwrap();
        assert_eq!(head.value, 5);
        assert!(head.next.is_none());
    }

    #[test]
    fn pop() {
        let mut stack: Stack<i32> = Stack::new();

        stack.push(5);
        stack.push(6);

        assert!(stack.head.is_some());
        assert_eq!(stack.len(), 2);

        let value = stack.pop();
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), 6);
        assert_eq!(stack.len(), 1);

        let value = stack.pop();
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), 5);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn pop_empty() {
        let mut stack: Stack<i32> = Stack::new();

        assert!(stack.head.is_none());
        assert_eq!(stack.len(), 0);

        let value = stack.pop();
        assert!(value.is_err());
        assert_eq!(value.unwrap_err(), DSError::StackEmpty);
    }
}
