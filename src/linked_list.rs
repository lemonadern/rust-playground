use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(), // いけてるか？
        }));

        self.head = Some(new_node);
    }

    pub fn append(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node { value, next: None }));
        match self.head.clone() {
            None => self.head = Some(new_node),
            Some(head) => {
                let mut current = head;
                while let Some(next_node) = {
                    // ブロックを作り、nextという一時変数をつくって返すことで
                    // メインのwhileブロックの中でcurrentを扱えるようにする
                    let next = current.borrow().next.clone();
                    next
                } {
                    current = next_node;
                }
                current.borrow_mut().next = Some(new_node);
            }
        }
    }
}

impl<T: std::fmt::Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let node = node.borrow();
            write!(f, "{:?}", node.value)?;
            write!(f, " -> ")?;
            current = node.next.clone();
        }
        write!(f, "None")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let list: LinkedList<isize> = LinkedList::new();
        assert_eq!(format!("{}", list), "None");
    }

    #[test]
    fn prepend_once() {
        let mut list = LinkedList::new();
        list.prepend(1);
        assert_eq!(format!("{}", list), "1 -> None");
    }

    #[test]
    fn prepend_twice() {
        let mut list = LinkedList::new();
        list.prepend(1);
        list.prepend(2);
        assert_eq!(format!("{}", list), "2 -> 1 -> None");
    }

    #[test]
    fn prepend_thrice() {
        let mut list = LinkedList::new();
        list.prepend(1);
        list.prepend(2);
        list.prepend(3);
        assert_eq!(format!("{}", list), "3 -> 2 -> 1 -> None");
    }

    #[test]
    fn append_when_list_is_empty() {
        let mut list = LinkedList::new();
        list.append(1);
        assert_eq!(format!("{}", list), "1 -> None");
    }

    #[test]
    fn append_twice() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        assert_eq!(format!("{}", list), "1 -> 2 -> None");
    }

    #[test]
    fn append_thrice() {
        let mut list = LinkedList::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(format!("{}", list), "1 -> 2 -> 3 -> None");
    }

    #[test]
    fn append_and_prepend() {
        let mut list = LinkedList::new();
        list.append(1);
        list.prepend(2);
        list.append(3);
        list.prepend(4);
        assert_eq!(format!("{}", list), "4 -> 2 -> 1 -> 3 -> None");
    }
}
