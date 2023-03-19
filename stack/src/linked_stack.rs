use crate::Stack;

struct Node <T>  {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedStack<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Default for LinkedStack<T> {
    fn default() -> Self {
        Self { head: Default::default(), size: 0 }
    }
}

impl<T: Copy> Stack<T> for LinkedStack<T> {
    fn push(&mut self, t: T) {
        if self.is_empty() {
            self.head = Some(Box::new(Node{value: t, next: None}));
        }else{
            self.head = Some(Box::new(Node{value: t, next: self.head.take()}));
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        }else{
            let current_head = self.head.as_mut().unwrap();
            let to_return = current_head.value;
            self.head = current_head.next.take();
            self.size -= 1;
            Some(to_return)
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        }else{
            Some(&self.head.as_ref().unwrap().value)
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0 
    }

    fn size (&self) -> usize {
        self.size
    }
}