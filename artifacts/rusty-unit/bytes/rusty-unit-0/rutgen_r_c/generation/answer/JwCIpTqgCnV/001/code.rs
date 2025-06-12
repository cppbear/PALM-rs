// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_exceeding_capacity() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    unsafe {
        buffer.set_len(10); // Fill the buffer to its capacity
        buffer.advance_mut(1); // This should trigger a panic as remaining is 0
    }
}

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_exact_capacity() {
    let mut buffer: Vec<u8> = Vec::with_capacity(5);
    unsafe {
        buffer.set_len(5); // Fill the buffer to its capacity
        buffer.advance_mut(5); // This should trigger a panic as remaining is 0
    }
} 

#[test]
#[should_panic]
fn test_advance_mut_panic_due_to_overflow() {
    let mut buffer: Vec<u8> = Vec::with_capacity(8);
    unsafe {
        buffer.set_len(7); // Buffer length is 7, remaining is 1
        buffer.advance_mut(2); // This should trigger a panic as remaining is 1
    }
}

#[test]
fn test_advance_mut_no_panic() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    unsafe {
        buffer.set_len(5); // Set length to 5
        buffer.advance_mut(3); // This should work as remaining is 5
        assert_eq!(buffer.len(), 8); // Length should now be 8
    }
}

