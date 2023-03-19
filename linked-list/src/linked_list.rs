use std::fmt;

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Node<T>>,
    size: usize,
}

impl<T: std::fmt::Display> LinkedList<T> {
    
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {

        if self.size == 0 {
            self.head = Some(Node { value, next: None});
        }else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(Node{value, next: None}));
        }
        self.size += 1;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
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

