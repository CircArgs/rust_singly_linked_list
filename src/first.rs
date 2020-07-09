#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, next: None }
    }
    pub fn append(&mut self, next: Node<T>) {
        match self.next.as_mut() {
            None => self.next = Some(Box::new(next)),
            Some(cur_next) => cur_next.append(next),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct List<T> {
    head: Option<Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(head) => head.append(new_node),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        Node::new(1);
    }

    #[test]
    fn test_append() {
        let mut list = List::new();
        list.append(1);
        list.append(2);
        assert_eq!(
            list,
            List {
                head: Some(Node {
                    value: 1,
                    next: Some(Box::new(Node {
                        value: 2,
                        next: None
                    }))
                })
            }
        )
    }
}
