use std::sync::{Arc, Mutex};
use std::mem;

pub struct RingBuffer<T, const N: usize> {
    storage: Arc<Mutex<[Option<T>; N]>>,
    head: usize,
    tail: usize,
    max_size: usize,
    full: bool,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        assert!(N > 1, "RingBuffer size must be greater than 1");
        Self {
            storage: Arc::new(Mutex::new([(); N].map(|_| None))),
            head: 0,
            tail: 0,
            max_size: N,
            full: false,
        }
    }

    pub fn reset(&mut self) {
        let _guard = self.storage.lock().unwrap();
        self.head = self.tail;
        self.full = false;
    }

    pub fn empty(&self) -> bool {
        let _guard = self.storage.lock().unwrap();
        !self.full && (self.head == self.tail)
    }

    pub fn full(&self) -> bool {
        let _guard = self.storage.lock().unwrap();
        self.full
    }

    pub fn capacity(&self) -> usize {
        self.max_size
    }

    pub fn size(&self) -> usize {
        let _guard = self.storage.lock().unwrap();
        if self.full {
            self.max_size
        } else if self.head >= self.tail {
            self.head - self.tail
        } else {
            self.max_size.wrapping_add(self.head).wrapping_sub(self.tail)
        }
    }

    pub fn put(&mut self, item: T) {    
        let mut data = self.storage.lock().unwrap();
        data[self.head] = Some(item);
        if self.full {
            self.tail = (self.tail + 1) % self.max_size;
        }
        self.head = (self.head + 1) % self.max_size;
        self.full = self.head == self.tail;
    }

    pub fn get(&mut self) -> Option<T> {
        let mut data = self.storage.lock().unwrap();
        if !self.full && self.head == self.tail {
            return None;
        }
        let item = mem::replace(&mut data[self.tail], None);
        self.full = false;
        self.tail = (self.tail + 1) % self.max_size;
        item
    }
}