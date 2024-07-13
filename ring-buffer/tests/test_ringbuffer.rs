#[cfg(test)]
mod tests {
    use ring_buffer::RingBuffer;

    
    #[test]
    fn test_new_ring_buffer() {
        let rb: RingBuffer<i32, 5> = RingBuffer::new();
        assert!(rb.empty());
        assert!(!rb.full());
        assert_eq!(rb.capacity(), 5);
        assert_eq!(rb.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
        rb.put(1);
        rb.put(2);
        assert_eq!(rb.get(), Some(1));
        assert_eq!(rb.get(), Some(2));
        assert_eq!(rb.get(), None);
    }

    #[test]
    fn test_overflow() {
        let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
        rb.put(1);
        rb.put(2);
        rb.put(3);
        rb.put(4);
        assert_eq!(rb.get(), Some(2));
        assert_eq!(rb.get(), Some(3));
        assert_eq!(rb.get(), Some(4));
        assert_eq!(rb.get(), None);
    }

    #[test]
    fn test_full_and_empty() {
        let mut rb: RingBuffer<i32, 2> = RingBuffer::new();
        assert!(rb.empty());
        assert!(!rb.full());

        rb.put(1);
        assert!(!rb.empty());
        assert!(!rb.full());

        rb.put(2);
        assert!(!rb.empty());
        assert!(rb.full());

        rb.get();
        assert!(!rb.empty());
        assert!(!rb.full());

        rb.get();
        assert!(rb.empty());
        assert!(!rb.full());
    }

    #[test]
    fn test_size() {
        let mut rb: RingBuffer<i32, 5> = RingBuffer::new();
        assert_eq!(rb.size(), 0);

        rb.put(1);
        assert_eq!(rb.size(), 1);

        rb.put(2);
        rb.put(3);
        assert_eq!(rb.size(), 3);

        rb.get();
        assert_eq!(rb.size(), 2);

        rb.put(4);
        rb.put(5);
        rb.put(6);
        assert_eq!(rb.size(), 5);
    }

    #[test]
    fn test_reset() {
        let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
        rb.put(1);
        rb.put(2);
        assert_eq!(rb.size(), 2);

        rb.reset();
        assert!(rb.empty());
        assert_eq!(rb.size(), 0);

        rb.put(3);
        assert_eq!(rb.get(), Some(3));
    }

    #[test]
    fn test_multiple_cycles() {
        let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
        for i in 0..10 {
            rb.put(i);
            if i >= 2 {
                assert_eq!(rb.get(), Some(i - 2));
            }
        }
        assert_eq!(rb.get(), Some(8));
        assert_eq!(rb.get(), Some(9));
        assert_eq!(rb.get(), None);
    }

    #[test]
    fn test_with_non_copy_type() {
        let mut rb: RingBuffer<String, 3> = RingBuffer::new();
        rb.put(String::from("hello"));
        rb.put(String::from("world"));
        assert_eq!(rb.get(), Some(String::from("hello")));
        assert_eq!(rb.get(), Some(String::from("world")));
    }

    #[test]
    #[should_panic(expected = "RingBuffer size must be greater than 1")]
    fn test_invalid_size() {
        let _rb: RingBuffer<i32, 1> = RingBuffer::new();
    }

    #[test]
    fn test_put_get_interleaved() {
        let mut rb: RingBuffer<i32, 3> = RingBuffer::new();
        rb.put(1);
        rb.put(2);
        assert_eq!(rb.get(), Some(1));
        rb.put(3);
        assert_eq!(rb.get(), Some(2));
        rb.put(4);
        assert_eq!(rb.get(), Some(3));
        assert_eq!(rb.get(), Some(4));
        assert_eq!(rb.get(), None);
    }
}