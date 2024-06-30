use std::{cell::RefCell, fmt::Display, rc::Rc};
use thiserror::Error;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    // 参照を辿ってハンドリングするトレーニングをしたいので、headのみを保持する
    // tail, length などは持たない
    head: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Error, Debug, PartialEq)]
pub enum InsertionError {
    #[error("Index is Too Big. Size of List is {size}, but you specified index {index}.")]
    TooBigIndex { size: usize, index: usize },
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // 先頭に値を追加
    pub fn prepend(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(), // いけてるか？
        }));

        self.head = Some(new_node);
    }

    // 末尾に値を追加
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

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(old_head) => {
                self.head = old_head.borrow().next.clone();

                match Rc::try_unwrap(old_head) {
                    Ok(v) => Some(v.into_inner().value),
                    Err(_) => unreachable!(),
                }
            }
        }
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), InsertionError> {
        match self.head.clone() {
            None => {
                if index > 0 {
                    return Err(InsertionError::TooBigIndex { size: 0, index });
                }
                let new_node = Rc::new(RefCell::new(Node { value, next: None }));
                self.head = Some(new_node);
                Ok(())
            }
            Some(head) => {
                if index == 0 {
                    // replace head with new element
                    let next = head;

                    let new_node = Rc::new(RefCell::new(Node {
                        value,
                        next: Some(next),
                    }));
                    self.head = Some(new_node);
                } else {
                    let mut current_node = head;
                    let mut current_index = 1;

                    while let Some(next_node) = {
                        let tmp = current_node.borrow().next.clone();
                        tmp
                    } {
                        if current_index == index {
                            // insert between existing nodes
                            let new_node = Rc::new(RefCell::new(Node {
                                value,
                                next: Some(next_node),
                            }));
                            current_node.borrow_mut().next = Some(new_node);
                            return Ok(());
                        }

                        current_node = next_node;
                        current_index += 1;
                    }

                    let list_size = current_index;
                    if index == list_size {
                        // insert to tail (append)
                        let new_node = Rc::new(RefCell::new(Node { value, next: None }));
                        current_node.borrow_mut().next = Some(new_node);
                        return Ok(());
                    } else if index > list_size {
                        return Err(InsertionError::TooBigIndex {
                            size: list_size,
                            index,
                        });
                    }
                }
                Ok(())
            }
        }
    }

    pub fn delete(&mut self, _index: usize) -> Result<(), &'static str> {
        todo!("implement delete");
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

    #[test]
    fn pop() {
        let mut list:LinkedList<i32> = LinkedList::new();
        list.prepend(1);
        list.prepend(2);
        list.prepend(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn list_is_empty_and_insert_to_index_zero() {
        let mut list = LinkedList::new();
        list.insert(0, 1).unwrap();
        assert_eq!(format!("{}", list), "1 -> None");
    }

    #[test]
    fn try_insert_by_nonzero_index_to_empty_list() {
        let mut list = LinkedList::new();
        let result = list.insert(1, 1);
        assert!(result.is_err());
    }

    #[test]
    fn insert_to_index_zero() {
        let mut list = LinkedList::new();
        list.prepend(1);
        list.prepend(0);
        list.insert(0, 9).unwrap();
        assert_eq!(format!("{}", list), "9 -> 0 -> 1 -> None");
    }

    #[test]
    fn insert_to_between_nodes_pattern_1() {
        let mut list = LinkedList::new();
        list.prepend(2);
        list.prepend(1);
        list.prepend(0);

        assert_eq!(format!("{}", list), "0 -> 1 -> 2 -> None");
        list.insert(1, 9).unwrap();
        assert_eq!(format!("{}", list), "0 -> 9 -> 1 -> 2 -> None");
    }

    #[test]
    fn insert_to_between_nodes_pattern_2() {
        let mut list = LinkedList::new();
        list.prepend(2);
        list.prepend(1);
        list.prepend(0);

        assert_eq!(format!("{}", list), "0 -> 1 -> 2 -> None");
        list.insert(2, 9).unwrap();
        assert_eq!(format!("{}", list), "0 -> 1 -> 9 -> 2 -> None");
    }

    #[test]
    fn insert_to_tail() {
        let mut list = LinkedList::new();
        list.prepend(0);

        assert_eq!(format!("{}", list), "0 -> None");
        list.insert(1, 9).unwrap();
        assert_eq!(format!("{}", list), "0 -> 9 -> None");
    }

    #[test]
    fn insert_to_tail_big() {
        let mut list = LinkedList::new();
        list.prepend(3);
        list.prepend(2);
        list.prepend(1);
        list.prepend(0);

        assert_eq!(format!("{}", list), "0 -> 1 -> 2 -> 3 -> None");
        list.insert(4, 9).unwrap();
        assert_eq!(format!("{}", list), "0 -> 1 -> 2 -> 3 -> 9 -> None");
    }

    #[test]
    fn insert_to_bigger_index_than_list_size() {
        let mut list = LinkedList::new();
        list.prepend(0);

        assert_eq!(format!("{}", list), "0 -> None");
        let result = list.insert(2, 9);
        assert!(result.is_err());
    }

    #[test]
    fn insert_to_bigger_index_than_list_size_big() {
        let mut list = LinkedList::new();
        list.prepend(3);
        list.prepend(2);
        list.prepend(1);
        list.prepend(0);

        assert_eq!(format!("{}", list), "0 -> 1 -> 2 -> 3 -> None");
        let result = list.insert(5, 9);
        assert!(result.is_err());
    }
}
