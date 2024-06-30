extern crate rust_playground;

use rust_playground::linked_list::LinkedList;

#[test]
fn construction() {
    let l: LinkedList<isize> = LinkedList::new();
    assert_eq!(format!("{}", l), "None");
}

#[test]
fn push_front() {
    let mut l = LinkedList::new();
    l.push_front(1);
    assert_eq!(format!("{}", l), "1 -> None");
    l.push_front(2);
    assert_eq!(format!("{}", l), "2 -> 1 -> None");
}

#[test]
fn pop_front() {
    let mut l = LinkedList::new();

    assert_eq!(l.pop_front(), None);

    l.push_front(1);
    l.push_front(2);

    assert_eq!(l.pop_front(), Some(2));
    assert_eq!(l.pop_front(), Some(1));
    assert_eq!(l.pop_front(), None);
}

#[test]
fn push_back() {
    let mut l = LinkedList::new();
    l.push_back(1);
    assert_eq!(format!("{}", l), "1 -> None");
    l.push_back(2);
    assert_eq!(format!("{}", l), "1 -> 2 -> None");
}

#[test]
fn pop_back() {
    let mut l = LinkedList::new();

    // returns `None` if a list is empty
    assert_eq!(l.pop_back(), None);

    // just one element
    l.push_front(1);
    assert_eq!(l.pop_back(), Some(1));
    assert_eq!(l.pop_back(), None);

    // two element
    l.push_front(1);
    l.push_front(2);

    assert_eq!(l.pop_back(), Some(1));
    assert_eq!(l.pop_back(), Some(2));
    assert_eq!(l.pop_back(), None);
}
