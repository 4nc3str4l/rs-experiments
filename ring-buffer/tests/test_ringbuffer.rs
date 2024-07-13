use ring_buffer::RingBuffer;

#[test]
fn test_ring_buffer_construct() {
    let rb = RingBuffer::<i32, 15>::new();
    assert_eq!(rb.capacity(), 15, "Ring buffer capacity should be 15");
}

#[test]
fn test_with_wrong_size() {
    let rb = RingBuffer::<String, 10>::new();
    assert_ne!(rb.capacity(), 0, "Ring buffer capacity shouldn't be 0");
}

#[test]
fn test_put_1_item() {
    let mut rb = RingBuffer::<String, 2>::new();
    let val = rb.put("Hello World!".to_owned());
    assert_eq!(val, 1);
}



#[test]
fn test_buffer_overflow() {
    let mut rb = RingBuffer::<u32, 2>::new();
    assert_eq!(1, rb.put(1));
    assert_eq!(1, rb.put(1));
    
}