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

    /// Adds an element first in the list.
    pub fn push_front(&mut self, value: T) {
        let n = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = Some(n);
    }

    /// Removes the first element and returns it, or `None` if the list is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None, // list is empty
            Some(old_head) => {
                self.head = old_head.borrow().next.clone();

                assert!(Rc::strong_count(&old_head) == 1);
                match Rc::try_unwrap(old_head) {
                    Ok(v) => Some(v.into_inner().value),
                    Err(_) => unreachable!(),
                }
            }
        }
    }

    /// Appends an element to the back of a list.
    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node { value, next: None }));
        match self.head.clone() {
            None => {
                self.head = Some(new_node);
            }
            Some(head) => {
                let mut current = head;

                // proceed while next element isn't `None`
                while let Some(next_node) = {
                    let next = current.borrow().next.clone();
                    next
                } {
                    current = next_node;
                }

                // block for assertion
                {
                    let next = current.borrow().next.clone();
                    assert!(next.is_none());
                }

                current.borrow_mut().next = Some(new_node);
            }
        }
    }

    /// Removes the last element from a list and returns it, or `None` if the list is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        match self.head.clone() {
            None => None, // list is empty.
            Some(head) => {
                // dbg!(Rc::strong_count(&head)); // 2

                // list has only one element
                if head.borrow().next.is_none() {
                    // dbg!(Rc::strong_count(&head)); // 2
                    self.head = head.borrow().next.clone();

                    assert!(Rc::strong_count(&head) == 1);
                    match Rc::try_unwrap(head) {
                        Ok(v) => Some(v.into_inner().value),
                        Err(_) => unreachable!(),
                    }
                } else {
                    // list has multiple element
                    let mut prev = None;
                    let mut current = head;
                    while let Some(node) = {
                        let next = current.borrow().next.clone();
                        next
                    } {
                        prev = Some(current);
                        current = node;
                    }

                    if let Some(prev_node) = prev {
                        prev_node.borrow_mut().next = None;
                    } else {
                        unreachable!();
                    }

                    assert!(Rc::strong_count(&current) == 1);
                    match Rc::try_unwrap(current) {
                        Ok(v) => Some(v.into_inner().value),
                        Err(_) => unreachable!(),
                    }
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
