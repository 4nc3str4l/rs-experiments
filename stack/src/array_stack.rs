use crate::Stack;

pub struct ArrayStack<T> {
    data: Vec<T>,
}

impl<T> Default for ArrayStack<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T> Stack<T> for ArrayStack<T> {
    fn push(&mut self, t: T) {
        self.data.push(t)
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    #[inline]
    fn size(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}
