use std::vec;

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

#[test]
fn test_index_of_in_empty() {
    let mut list = LinkedList::<u32>::new();
    assert_eq!(list.index_of(1), None);
}

#[test]
fn test_index_of_first() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
    }
    assert_eq!(list.index_of(0).unwrap(), 0);
}

#[test]
fn test_index_of_last() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
    }
    assert_eq!(list.index_of(4).unwrap(), 4);
}

#[test]
fn test_index_of_middle() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
    }
    assert_eq!(list.index_of(2).unwrap(), 2);
}

#[test]
fn test_index_of_non_existent() {
    let mut list = LinkedList::<u32>::new();
    for i in 0..5 {
        list.insert(i, i as usize);
    }
    assert_eq!(list.index_of(12), None);
}

#[test]
fn test_index_of_non_existent_1_element() {
    let mut list = LinkedList::<u32>::new();
    list.add(1);
    assert_eq!(list.index_of(12), None);
}

#[test]
fn test_index_of_existent_1_element() {
    let mut list = LinkedList::<u32>::new();
    list.add(1);
    assert_eq!(list.index_of(1).unwrap(), 0);
}

#[test]
fn test_get_head_in_empty() {
    let list = LinkedList::<u32>::new();
    assert_eq!(list.get_head(), None);
}

#[test]
fn test_get_head_in_full() {
    let mut list = LinkedList::<u32>::new();
    list.add(2);
    assert_eq!(list.get_head(), Some(2));
}

#[test]
fn test_get_head_after_remove_at_empty() {
    let mut list = LinkedList::<u32>::new();
    list.add(2);
    assert_eq!(list.get_head(), Some(2));
    list.remove_at(0);
    assert_eq!(list.get_head(), None);
}

#[test]
fn test_get_head_after_remove_at_some() {
    let mut list = LinkedList::<u32>::new();
    list.add(2);
    assert_eq!(list.get_head(), Some(2));
    list.add(3);
    list.remove_at(0);
    assert_eq!(list.get_head(), Some(3));
}

#[test]
fn test_get_head_after_remove_empty() {
    let mut list = LinkedList::<u32>::new();
    list.add(2);
    assert_eq!(list.get_head(), Some(2));
    list.remove(2);
    assert_eq!(list.get_head(), None);
}

#[test]
fn test_get_head_after_remove_some() {
    let mut list = LinkedList::<u32>::new();
    list.add(2);
    assert_eq!(list.get_head(), Some(2));
    list.add(3);
    list.remove(2);
    assert_eq!(list.get_head(), Some(3));
}

#[test]
fn test_get_head_after_insert_at_first() {
    let mut list = LinkedList::<u32>::new();
    list.add(2);
    assert_eq!(list.get_head(), Some(2));
    list.insert(3, 0);
    assert_eq!(list.get_head(), Some(3));
}

#[test]
fn test_get_at_in_empty() {
    let list = LinkedList::<String>::new();
    assert_eq!(list.get_at(0), None);
}

#[test]
fn test_get_at_first_in_1_element() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    assert_eq!(list.get_at(0).unwrap(), "Rust");
}

#[test]
fn test_get_at_outside_in_1_element() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    assert_eq!(list.get_at(1), None);
    assert_eq!(list.get_at(2), None);
}

#[test]
fn test_get_at_last() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    list.add("is".to_owned());
    list.add("awesome".to_owned());
    assert_eq!(list.get_at(list.len() - 1).unwrap(), "awesome");
}

#[test]
fn test_get_at() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    list.add("is".to_owned());
    list.add("awesome".to_owned());
    assert_eq!(list.get_at(0).unwrap(), "Rust");
    assert_eq!(list.get_at(1).unwrap(), "is");
    assert_eq!(list.get_at(2).unwrap(), "awesome");
    assert_eq!(list.get_at(3), None);
}

#[test]
fn test_get_tail_at_empty() {
    let list = LinkedList::<String>::new();
    assert_eq!(list.get_tail(), None);
}

#[test]
fn test_get_tail_at_single() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    assert_eq!(list.get_tail().unwrap(), "Rust");
    assert_eq!(list.get_tail().unwrap(), list.get_head().unwrap());
}

#[test]
fn test_get_tail_at_two() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    list.add("awesome".to_owned());
    assert_eq!(list.get_head().unwrap(), "Rust");
    assert_eq!(list.get_tail().unwrap(), "awesome");
}

#[test]
fn test_get_tail_at_n() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    list.add("is".to_owned());
    list.add("awesome".to_owned());
    assert_eq!(list.get_head().unwrap(), "Rust");
    assert_eq!(list.get_tail().unwrap(), "awesome");
}

#[test]
fn test_get_tail_after_remove() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    list.add("is".to_owned());
    list.add("awesome".to_owned());
    assert_eq!(list.get_head().unwrap(), "Rust");
    assert_eq!(list.get_tail().unwrap(), "awesome");
    list.remove_at(list.len() - 1);
    assert_eq!(list.get_tail().unwrap(), "is");
}

#[test]
fn test_clear_in_empty() {
    let mut list = LinkedList::<String>::new();
    list.clear();
    assert!(list.is_empty());
    assert_eq!(list.get_head(), None);
    assert_eq!(list.get_tail(), None);
}

#[test]
fn test_clear_at_single() {
    let mut list = LinkedList::<String>::new();
    list.add("Rust".to_owned());
    list.clear();
    assert!(list.is_empty());
    assert_eq!(list.get_head(), None);
    assert_eq!(list.get_tail(), None);
}

#[test]
fn test_clear_at_n() {
    let mut list = LinkedList::<String>::default();
    list.add("Rust".to_owned());
    list.add("is".to_owned());
    list.add("awesome".to_owned());
    list.clear();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert_eq!(list.get_head(), None);
    assert_eq!(list.get_tail(), None);
}

#[test]
fn test_collect() {
    let list = vec![1, 2, 3].into_iter().collect::<LinkedList<i32>>();
    assert_eq!(list.len(), 3);
    assert_eq!(list.get_head(), Some(1));
    assert_eq!(list.get_tail(), Some(3));
}

#[test]
fn test_from_vec() {
    let list = LinkedList::from_vec(vec![1, 2, 3]);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get_head(), Some(1));
    assert_eq!(list.get_tail(), Some(3));
}

#[test]
fn test_from_vec_and_back() {
    let list = LinkedList::from_vec(vec![1, 2, 3]);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get_head(), Some(1));
    assert_eq!(list.get_tail(), Some(3));
    let vector = list.into_iter().collect::<Vec<i32>>();
    assert_eq!(vector, vec![1, 2, 3]);
}

#[test]
fn test_from_vec_and_back_clone() {
    let list = LinkedList::from_vec(vec![1, 2, 3]);
    let vector = list.clone().into_iter().collect::<Vec<i32>>();
    assert_eq!(vector, vec![1, 2, 3]);
    assert_eq!(list.len(), 3);
    assert_eq!(list.get_head(), Some(1));
    assert_eq!(list.get_tail(), Some(3));
}
