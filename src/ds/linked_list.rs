use super::DSError;

#[allow(unused)]
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[allow(unused)]
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

#[allow(unused)]
impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn insert(&mut self, value: T) {
        let mut node = Box::new(Node::new(value));

        let head = self.head.take();
        if head.is_some() {
            node.next = head;
        }
        self.head = Some(node);

        self.length += 1;
    }

    pub fn insert_at(&mut self, value: T, index: usize) -> Result<(), DSError> {
        if index > self.length - 1 {
            return Err(DSError::LinkedListError);
        } else if index == 0 {
            self.insert(value);
            return Ok(());
        }

        let mut current = &mut self.head;

        for i in 1..=index {
            if i == index {
                break;
            }

            let Some(c) = current else {
                return Err(DSError::LinkedListError);
            };

            current = &mut c.next;
        }

        current
            .as_mut()
            .map(|c| {
                let next = c.next.take();
                let mut node = Node::new(value);
                node.next = next;
                c.next = Some(Box::new(node));

                self.length += 1;
            })
            .ok_or(DSError::LinkedListError)?;

        Ok(())
    }

    pub fn delete(&mut self, node: Node<T>) -> T {
        todo!();
    }

    pub fn delete_at(&mut self, index: usize) -> T {
        todo!();
    }

    pub fn get(&mut self, index: usize) -> T {
        todo!();
    }

    pub fn len(&self) -> usize {
        self.length
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

        ll.insert(5);

        assert!(ll.head.is_some());
        assert_eq!(ll.length, 1);

        let head = ll.head.unwrap();
        assert_eq!(head.value, 5);
        assert!(head.next.is_none());
    }

    #[test]
    fn insert_multiple() {
        let mut ll: List<i32> = List::new();

        ll.insert(6);
        ll.insert(7);

        let head = ll.head.unwrap();
        assert_eq!(head.value, 7);
        assert!(head.next.is_some());

        let next = head.next.unwrap();
        assert_eq!(next.value, 6);
    }

    #[test]
    fn insert_at() {
        let mut ll: List<i32> = List::new();

        ll.insert(5);
        ll.insert(6);
        let created = ll.insert_at(7, 1);

        assert!(created.is_ok());
        assert_eq!(ll.len(), 3);
        assert_eq!(ll.head.unwrap().next.unwrap().value, 7);
    }
}
