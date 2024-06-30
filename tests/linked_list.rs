extern crate rust_playground;

use rust_playground::linked_list::LinkedList;

#[test]
fn construction() {
    let l: LinkedList<isize> = LinkedList::new();
    assert_eq!(format!("{}", l), "None");
}

mod push_front {
    use rust_playground::linked_list::LinkedList;

    #[test]
    fn example() {
        let mut l = LinkedList::new();
        assert_eq!(format!("{}", l), "None");
        l.push_front(1);
        assert_eq!(format!("{}", l), "1 -> None");
        l.push_front(2);
        assert_eq!(format!("{}", l), "2 -> 1 -> None");
    }

    #[test]
    fn push_once() {
        let mut l = LinkedList::new();
        l.push_front(1);
        assert_eq!(format!("{}", l), "1 -> None");
    }

    #[test]
    fn push_twice() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        assert_eq!(format!("{}", l), "2 -> 1 -> None");
    }

    #[test]
    fn push_thrice() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        assert_eq!(format!("{}", l), "3 -> 2 -> 1 -> None");
    }
}

mod pop_front {
    use rust_playground::linked_list::LinkedList;

    #[test]
    fn example() {
        let mut l = LinkedList::new();
        l.push_front(1);
        l.push_front(2);
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), None);
    }

    #[test]
    fn returns_none_if_list_is_empty() {
        let mut l: LinkedList<i32> = LinkedList::new();
        assert_eq!(l.pop_front(), None);
    }
}

mod push_back {
    use rust_playground::linked_list::LinkedList;

    #[test]
    fn example() {
        let mut l = LinkedList::new();
        assert_eq!(format!("{}", l), "None");
        l.push_back(1);
        assert_eq!(format!("{}", l), "1 -> None");
        l.push_back(2);
        assert_eq!(format!("{}", l), "1 -> 2 -> None");
    }

    #[test]
    fn push_once() {
        let mut l = LinkedList::new();
        l.push_back(1);
        assert_eq!(format!("{}", l), "1 -> None");
    }

    #[test]
    fn push_twice() {
        let mut l = LinkedList::new();
        l.push_back(1);
        l.push_back(2);
        assert_eq!(format!("{}", l), "1 -> 2 -> None");
    }

    #[test]
    fn push_thrice() {
        let mut l = LinkedList::new();
        l.push_back(1);
        l.push_back(2);
        l.push_back(3);
        assert_eq!(format!("{}", l), "1 -> 2 -> 3 -> None");
    }
}

mod pop_back {
    use rust_playground::linked_list::LinkedList;

    #[test]
    fn example() {
        let mut l = LinkedList::new();

        l.push_front(1);
        l.push_front(2);

        assert_eq!(l.pop_back(), Some(1));
        assert_eq!(l.pop_back(), Some(2));
        assert_eq!(l.pop_back(), None);
    }

    #[test]
    fn returns_none_if_list_is_empty() {
        let mut l: LinkedList<i32> = LinkedList::new();

        assert_eq!(l.pop_back(), None);
    }

    #[test]
    fn just_one_element() {
        let mut l = LinkedList::new();

        l.push_front(1);
        assert_eq!(l.pop_back(), Some(1));
        assert_eq!(l.pop_back(), None);
    }

    #[test]
    fn multiple_elements() {
        let mut l = LinkedList::new();

        l.push_front(1);
        l.push_front(2);
        l.push_front(3);
        assert_eq!(format!("{}", l), "3 -> 2 -> 1 -> None");

        assert_eq!(l.pop_back(), Some(1));
        assert_eq!(l.pop_back(), Some(2));
        assert_eq!(l.pop_back(), Some(3));
        assert_eq!(l.pop_back(), None);
    }
}

mod insert {
    use rust_playground::linked_list::LinkedList;

    #[test]
    fn example() {
        let mut l = LinkedList::new();
        l.push_front(2);
        l.push_front(1);
        l.push_front(0);

        assert_eq!(format!("{}", l), "0 -> 1 -> 2 -> None");
        assert!(l.insert(1, 9).is_ok());
        assert_eq!(format!("{}", l), "0 -> 9 -> 1 -> 2 -> None");
    }

    #[test]
    fn insert_first() {
        let mut l = LinkedList::new();
        l.push_front(2);
        l.push_front(1);
        l.push_front(0);

        assert_eq!(format!("{}", l), "0 -> 1 -> 2 -> None");
        assert!(l.insert(0, 9).is_ok());
        assert_eq!(format!("{}", l), "9 -> 0 -> 1 -> 2 -> None");
    }

    #[test]
    fn insert_middle() {
        let mut l = LinkedList::new();
        l.push_front(2);
        l.push_front(1);
        l.push_front(0);

        assert_eq!(format!("{}", l), "0 -> 1 -> 2 -> None");
        assert!(l.insert(2, 9).is_ok());
        assert_eq!(format!("{}", l), "0 -> 1 -> 9 -> 2 -> None");
    }

    #[test]
    fn insert_last() {
        let mut l = LinkedList::new();
        l.push_front(2);
        l.push_front(1);
        l.push_front(0);

        assert_eq!(format!("{}", l), "0 -> 1 -> 2 -> None");
        assert!(l.insert(3, 9).is_ok());
        assert_eq!(format!("{}", l), "0 -> 1 -> 2 -> 9 -> None");
    }

    #[test]
    fn insert_out_of_bound() {
        let mut l = LinkedList::new();
        l.push_front(2);
        l.push_front(1);
        l.push_front(0);

        assert_eq!(format!("{}", l), "0 -> 1 -> 2 -> None");
        assert!(l.insert(4, 9).is_err());
        assert_eq!(format!("{}", l), "0 -> 1 -> 2 -> None");
    }

    #[test]
    fn insert_to_empty_list() {
        let mut l = LinkedList::new();
        assert_eq!(format!("{}", l), "None");
        assert!(l.insert(0, 1).is_ok());
        assert_eq!(format!("{}", l), "1 -> None");

        let mut l = LinkedList::new();
        assert_eq!(format!("{}", l), "None");
        assert!(l.insert(1, 1).is_err());
        assert_eq!(format!("{}", l), "None");
    }
}
