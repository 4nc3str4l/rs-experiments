pub struct RingBuffer<T, const N: usize> {
    storage: [Option<T>; N],
    read_pos: usize,
    write_pos: usize,
}

impl<T: Default, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        assert!(N > 1, "RingBuffer size must be greater than 1");
        Self {
            storage: [(); N].map(|_| None),
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn put(&mut self, item: T) -> usize {    
        if (self.write_pos + 1) % self.capacity() == self.read_pos {
            return 0;
        }
        self.storage[self.write_pos] = Some(item);
        self.write_pos = (self.write_pos + 1) % self.capacity();
        1
    }

    pub fn capacity(&self) -> usize {
        self.storage.len()
    }
}