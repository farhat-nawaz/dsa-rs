#[allow(unused)]
#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[allow(unused)]
impl<T: Default> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[allow(unused)]
#[derive(Debug, Default)]
struct List<T> {
    head: Option<Node<T>>,
    length: usize,
}

#[allow(unused)]
impl<T: Default> List<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, mut node: Node<T>) {
        if let Some(head) = self.head.take() {
            node.next = Some(Box::new(head));
            self.head = Some(node);
        } else {
            self.head = Some(node);
        }

        self.length += 1;
    }
}

mod tests {
    use super::{List, Node};

    #[test]
    fn create_list() {
        let ll: List<i32> = List::new();

        assert!(ll.head.is_none());
        assert_eq!(ll.length, 0);
    }

    #[test]
    fn insert() {
        let mut ll: List<i32> = List::new();
        let node = Node::new(5);

        ll.insert(node);

        assert!(ll.head.is_some());
        assert_eq!(ll.length, 1);

        let head = ll.head.unwrap();
        assert_eq!(head.value, 5);
        assert!(head.next.is_none());
    }

    #[test]
    fn insert_multiple() {
        let mut ll: List<i32> = List::new();
        let node = Node::new(6);
        let node_2 = Node::new(7);

        ll.insert(node);
        ll.insert(node_2);

        let head = ll.head.unwrap();
        assert_eq!(head.value, 7);
        assert!(head.next.is_some());

        let next = head.next.unwrap();
        assert_eq!(next.value, 6);
    }
}
