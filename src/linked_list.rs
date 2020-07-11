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

#[derive(Debug, PartialEq)]
pub struct ListIter<T> {
    current: Option<Box<Node<T>>>,
}

impl<T> Iterator for ListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next;
            node.value
        })
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        if let Some(head) = self.head {
            ListIter {
                current: Some(Box::new(head)),
            }
        } else {
            ListIter { current: None }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BorrowedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for BorrowedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.current.take()?;

        self.current = temp
            .next
            .as_ref()
            .map(|borrowed_boxed_node| &**borrowed_boxed_node);
        Some(&temp.value)
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = BorrowedListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        BorrowedListIter {
            current: self.head.as_ref(),
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

    #[test]
    fn test_fold() {
        let mut list = List::new();
        list.append(1);
        list.append(2);
        list.append(4);
        list.append(5);
        assert_eq!(list.into_iter().fold(0, |x, y| x + y), 12);
    }
    #[test]
    fn test_empty_iter() {
        let list: List<i32> = List::new();
        assert_eq!(list.into_iter().count(), 0);
    }
    #[test]
    fn test_fold_borrowed() {
        let mut list = List::new();
        list.append(1);
        list.append(2);
        list.append(4);
        list.append(5);
        assert_eq!((&list).into_iter().fold(0, |x, y| x + y), 12);
    }
}
