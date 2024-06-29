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

mod tests {
    use super::*;

    #[test]
    fn construction() {
        let list: LinkedList<isize> = LinkedList::new();
        assert_eq!(format!("{}", list), "None");
    }

    #[test]
    fn prepend_1() {
        let mut list = LinkedList::new();
        list.prepend(1);
        assert_eq!(format!("{}", list), "1 -> None");
    }

    #[test]
    fn prepend_2() {
        let mut list = LinkedList::new();
        list.prepend(1);
        list.prepend(2);
        assert_eq!(format!("{}", list), "2 -> 1 -> None");
    }
}
