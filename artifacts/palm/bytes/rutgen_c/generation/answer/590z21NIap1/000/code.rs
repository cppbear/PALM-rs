// Answer 0

#[test]
fn test_advance_unchecked_no_op() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    unsafe {
        bytes_mut.advance_unchecked(0);
    }
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(bytes_mut.cap, 10);
}

#[test]
fn test_advance_unchecked_advance() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    // Assume the length is 10 for this test context
    unsafe {
        bytes_mut.set_len(10);
        bytes_mut.advance_unchecked(5);
    }
    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(bytes_mut.cap, 10);
}

#[test]
#[should_panic(expected = "internal: set_start out of bounds")]
fn test_advance_unchecked_exceeds_capacity() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    // Assume the length is initialized to 10 for this test
    unsafe {
        bytes_mut.set_len(10);
        bytes_mut.cap = 10; // setting capacity
        bytes_mut.advance_unchecked(15); // exceeds capacity, should panic
    }
}

#[test]
fn test_advance_unchecked_promotion() {
    // Tests to ensure promotion occurs correctly when advancing beyond limits
    let mut bytes_mut = unsafe { BytesMut::with_capacity(10) };
    // Assume we set the length to 10 and promote to shared if needed
    unsafe {
        bytes_mut.set_len(10);
        bytes_mut.cap = 10;
        bytes_mut.advance_unchecked(10); // This should trigger promotion
    }
    assert_eq!(bytes_mut.len(), 0); // length post advance
    assert!(bytes_mut.cap < 10); // capacity should be deducted
}

