use std::{fmt, cmp::PartialEq};

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: std::fmt::Display + PartialEq + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn add(&mut self, value: T) {
        if self.size == 0 {
            self.head = Some(Box::new(Node { value, next: None }));
        } else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(Node { value, next: None }));
        }
        self.size += 1;
    }

    pub fn insert(&mut self, value: T, position: usize) {
        if self.size == 0 {
            self.head = Some(Box::new(Node { value, next: None }));
        } else if position == 0 {
            self.head = Some(Box::new(Node {
                value,
                next: Some(self.head.take().unwrap()),
            }));
        } else if position >= self.size {
            self.add(value);
            return;
        } else {
            let mut current = self.head.as_mut().unwrap();
            let mut counter = 0;
            while counter != (position - 1) {
                current = current.next.as_mut().unwrap();
                counter += 1;
            }
            current.next = Some(Box::new(Node {
                value,
                next: current.next.take(),
            }));
        }

        self.size += 1;
    }

    pub fn remove(&mut self, value: T) {
        if self.size == 0 {
            return;
        }
        let mut current = self.head.as_mut().unwrap();
        if current.value == value {
            if self.size > 1 {
                self.head = self.head.as_mut().unwrap().next.take();
            } else {
                self.head = None;
            }
            self.size -= 1;
            return;
        }

        while current.next.is_some() {
            let next = current.next.as_mut().unwrap();
            if next.value == value {
                current.next = next.next.take();
                self.size -= 1;
                break;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
    }

    pub fn remove_at(&mut self, position: usize) {
        if self.size == 0 {
            return;
        }

        if position == 0 {
            if self.size > 1 {
                self.head = self.head.as_mut().unwrap().next.take();
            } else {
                self.head = None;
            }
            self.size -= 1;
            return;
        }

        let mut current = self.head.as_mut().unwrap();
        let mut idx = 1;
        while current.next.is_some() {
            let next = current.next.as_mut().unwrap();
            if idx == position {
                current.next = next.next.take();
                self.size -= 1;
                break;
            } else {
                current = current.next.as_mut().unwrap();
                idx += 1;
            }
        }
    }

    pub fn index_of(&mut self, value: T) -> Option<usize> {
        if self.size == 0 {
            return None;
        } else {
            let mut current = self.head.as_mut().unwrap();
            if current.value == value {
                return Some(0);
            }

            let mut idx = 1;
            while current.next.is_some() {
                let next = current.next.as_mut().unwrap();
                if next.value == value {
                    return Some(idx);
                } else {
                    current = current.next.as_mut().unwrap();
                    idx += 1;
                }
            }
        }
        None
    }

    pub fn get_head(&self) -> Option<T> {
        self.head.as_ref().map(|head| head.value.clone())
    }

    pub fn get_at(&self, position: usize) -> Option<T> {
        if self.size == 0 {
            return None;
        } else {
            if position == 0 {
                return self.get_head();
            }

            let mut current = self.head.as_ref().unwrap();
            let mut idx = 1;
            while current.next.is_some() {
                let next = current.next.as_ref().unwrap();
                if idx == position {
                    return Some(next.value.clone());
                } else {
                    current = current.next.as_ref().unwrap();
                    idx += 1;
                }
            }
        }
        None
    }

    pub fn get_tail(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.get_at(self.len() - 1)
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.size = 0;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn from_vec(vector: Vec<T>) -> Self {
        vector.into_iter().collect()
    }
}

impl<T: std::fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.size == 0 {
            return write!(f, "[]");
        }

        let mut out = String::new();
        let mut current = self.head.as_ref().unwrap();
        out += "[";
        out += &format!("{}", &current.value);
        while current.next.is_some() {
            current = current.next.as_ref().unwrap();
            out += &format!(", {}", &current.value);
        }

        write!(f, "{}]", &out)
    }
}

impl<T: std::fmt::Display + PartialEq + Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}


impl<T: std::fmt::Display + PartialEq + Clone> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self{
        let mut list = LinkedList { head: None, size: 0 };

        for value in iter {
            list.add(value);
        }

        list
    }
}