use stack::{linked_stack::LinkedStack, Stack};

#[test]
fn initialize_stack() {
    let array_stack = LinkedStack::<u32>::default();
    assert_eq!(array_stack.size(), 0);
    assert_eq!(array_stack.is_empty(), true);
}

#[test]
fn test_push() {
    let mut array_stack = LinkedStack::<u32>::default();
    array_stack.push(1);
    array_stack.push(1);
    assert_eq!(array_stack.size(), 2);
}

#[test]
fn test_pop() {
    let mut array_stack = LinkedStack::<u32>::default();
    array_stack.push(1);
    array_stack.push(2);
    assert_eq!(array_stack.size(), 2);
    let first = array_stack.pop();
    assert_eq!(first.unwrap(), 2);
    assert_eq!(array_stack.size(), 1);
    let second = array_stack.pop();
    assert_eq!(second.unwrap(), 1);
    assert_eq!(array_stack.is_empty(), true);
    let third = array_stack.pop();
    assert_eq!(third, None);
}

#[test]
fn test_peek() {
    let mut array_stack = LinkedStack::<u32>::default();
    array_stack.push(1);
    array_stack.push(2);
    assert_eq!(array_stack.size(), 2);
    let first = array_stack.peek();
    assert_eq!(*first.unwrap(), 2);
    assert_eq!(array_stack.size(), 2);
    let second = array_stack.peek();
    assert_eq!(*second.unwrap(), 2);
    assert_eq!(array_stack.size(), 2);
}