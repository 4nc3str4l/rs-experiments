use std::fmt;

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: std::fmt::Display + std::cmp::PartialEq> LinkedList<T> {
    
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn add(&mut self, value: T) {

        if self.size == 0 {
            self.head = Some(Box::new(Node { value, next: None}));
        }else {
            let mut current = self.head.as_mut().unwrap();
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(Node{value, next: None}));
        }
        self.size += 1;
    }

    pub fn insert(&mut self, value: T, position: usize) {
        if self.size == 0 {
            self.head = Some(Box::new(Node { value, next: None}));
        }else {
            if position == 0 {
                self.head = Some(Box::new(Node { value, next: Some(self.head.take().unwrap())}));
            } else if position >= self.size {
                self.add(value);
                return;
            } else {
                let mut current = self.head.as_mut().unwrap();
                let mut counter = 0;
                while counter != (position -1) {
                    current = current.next.as_mut().unwrap();
                    counter += 1;
                }
                current.next = Some(Box::new(Node{value, next: current.next.take()}));
            }
        }
        self.size += 1;
    }

    pub fn remove(&mut self, value: T) {
        if self.size == 0 {
            return;
        }else {
            let mut current = self.head.as_mut().unwrap();
            if current.value == value{
                if self.size > 1 {
                    self.head = self.head.as_mut().unwrap().next.take();
                }else{
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
                }else{
                    current = current.next.as_mut().unwrap();
                }
            }
        }
    }

    pub fn remove_at(&mut self, position: usize) {
        if self.size == 0 {
            return;
        }else {
            if position == 0{
                if self.size > 1 {
                    self.head = self.head.as_mut().unwrap().next.take();
                }else{
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
                }else{
                    current = current.next.as_mut().unwrap();
                    idx += 1;
                }
            }
        }
    }

    pub fn index_of(&mut self, value: T) -> Option<usize> {
        None
    }

    pub fn get_head() -> Option<T> {
        None
    }

    pub fn get_at(&mut self, position: usize) -> Option<T> {
        None
    }

    pub fn get_tail() -> Option<T> {
        None
    }

    pub fn clear() {

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


#[cfg(test)]
mod tests {

    use super::LinkedList;

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
        assert_eq!(format!("{}", list), "[11, 0, 1, 2]", "Invalid list contents");
    }

    #[test]
    fn test_insert_at_end() {
        let mut list = LinkedList::<u32>::new();
        for i in 0..3 {
            list.add(i);
        }

        list.insert(11, 3);
        assert_eq!(list.len(), 4, "Invalid size");
        assert_eq!(format!("{}", list), "[0, 1, 2, 11]", "Invalid list contents");
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
        assert_eq!(format!("{}", list), "[0, 1, 2, 3, 4]", "Invalid list contents");
    }

    #[test]
    fn test_insert_as_stack() {
        let mut list = LinkedList::<u32>::new();
        for i in 0..5 {
            list.insert(i, 0);
            assert_eq!(list.len(), (i + 1) as usize, "Invalid size");
        }
        assert_eq!(format!("{}", list), "[4, 3, 2, 1, 0]", "Invalid list contents");
    }


    // DELETE

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
    fn test_remove_at_middle() {
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


    
}
