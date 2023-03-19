pub mod array_stack;
pub mod linked_stack;

pub trait Stack<T> {
    fn push(&mut self, t: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}
