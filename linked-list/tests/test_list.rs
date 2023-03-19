use linked_list::LinkedList;

#[test]
fn test_linked_list_construct() {
    let list = LinkedList::<u32>::new();
    assert_eq!(list.len(), 0, "List size should be 0");
    assert_eq!(format!("{}", list), "[]", "List should be empty");
}

#[test]
fn test_add_single_element() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    assert_eq!(format!("{}", list), "[Rust]", "Invalid list contents");
    assert_eq!(list.len(), 1, "List length should be 1");
}

#[test]
fn test_add_n_elements() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..100 {
        list.add(i);
        assert_eq!(list.len(), (i + 1) as usize, "Invalid size");
    }
}

#[test]
fn test_insert_at_beginning() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }
    list.insert(11, 0);
    assert_eq!(list.len(), 4, "Invalid size");
    assert_eq!(
        format!("{}", list),
        "[11, 0, 1, 2]",
        "Invalid list contents"
    );
}

#[test]
fn test_insert_at_end() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }

    list.insert(11, 3);
    assert_eq!(list.len(), 4, "Invalid size");
    assert_eq!(
        format!("{}", list),
        "[0, 1, 2, 11]",
        "Invalid list contents"
    );
}

#[test]
fn test_insert_at_middle() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..2 {
        list.add(i);
    }
    list.insert(11, 1);
    assert_eq!(list.len(), 3, "Invalid size");
    assert_eq!(format!("{}", list), "[0, 11, 1]", "Invalid list contents");
}

#[test]
fn test_insert_n_elements() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
        assert_eq!(list.len(), (i + 1) as usize, "Invalid size");
    }
    assert_eq!(
        format!("{}", list),
        "[0, 1, 2, 3, 4]",
        "Invalid list contents"
    );
}

#[test]
fn test_insert_as_stack() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, 0);
        assert_eq!(list.len(), (i + 1) as usize, "Invalid size");
    }
    assert_eq!(
        format!("{}", list),
        "[4, 3, 2, 1, 0]",
        "Invalid list contents"
    );
}

#[test]
fn test_remove_empty() {
    let mut list = LinkedList::<u32>::new();
    list.remove(100);
    assert_eq!(list.len(), 0, "Invalid size");
    assert_eq!(format!("{}", list), "[]", "Invalid list contents");
}

#[test]
fn test_remove_first() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }
    list.remove(0);
    assert_eq!(list.len(), 2, "Invalid size");
    assert_eq!(format!("{}", list), "[1, 2]", "Invalid list contents");
}

#[test]
fn test_remove_last() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }

    list.remove(2);
    assert_eq!(list.len(), 2, "Invalid size");
    assert_eq!(format!("{}", list), "[0, 1]", "Invalid list contents");
}

#[test]
fn test_remove_middle() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }
    list.remove(1);
    assert_eq!(list.len(), 2, "Invalid size");
    assert_eq!(format!("{}", list), "[0, 2]", "Invalid list contents");
}

#[test]
fn test_remove_n_elements() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
        assert_eq!(list.len(), (i + 1) as usize, "Invalid size");
    }

    let intial_length = list.len();

    for i in 0..5 {
        list.remove(i);
        assert_eq!(list.len(), intial_length - (i + 1) as usize, "Invalid size");
    }
    assert_eq!(format!("{}", list), "[]", "Invalid list contents");
}

#[test]
fn test_remove_at_empty() {
    let mut list = LinkedList::<u32>::new();
    list.remove_at(0);
    assert_eq!(list.len(), 0, "Invalid size");
    assert_eq!(format!("{}", list), "[]", "Invalid list contents");
}

#[test]
fn test_remove_at_first() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }
    list.remove_at(0);
    assert_eq!(list.len(), 2, "Invalid size");
    assert_eq!(format!("{}", list), "[1, 2]", "Invalid list contents");
}

#[test]
fn test_remove_at_last() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }

    list.remove_at(2);
    assert_eq!(list.len(), 2, "Invalid size");
    assert_eq!(format!("{}", list), "[0, 1]", "Invalid list contents");
}

#[test]
fn test_remove_at_middle() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..3 {
        list.add(i);
    }
    list.remove_at(1);
    assert_eq!(list.len(), 2, "Invalid size");
    assert_eq!(format!("{}", list), "[0, 2]", "Invalid list contents");
}

#[test]
fn test_remove_at_n_elements() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
        assert_eq!(list.len(), (i + 1) as usize, "Invalid size");
    }

    let intial_length = list.len();

    for i in 0..5 {
        list.remove_at(0);
        assert_eq!(list.len(), intial_length - (i + 1) as usize, "Invalid size");
    }
    assert_eq!(format!("{}", list), "[]", "Invalid list contents");
}

#[test]
fn test_remove_at_n_elements_last() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
        assert_eq!(list.len(), (i + 1) as usize, "Invalid size");
    }

    let intial_length = list.len();

    for i in 0..5 {
        list.remove_at(list.len() - 1);
        assert_eq!(list.len(), intial_length - (i + 1) as usize, "Invalid size");
    }
    assert_eq!(format!("{}", list), "[]", "Invalid list contents");
}
