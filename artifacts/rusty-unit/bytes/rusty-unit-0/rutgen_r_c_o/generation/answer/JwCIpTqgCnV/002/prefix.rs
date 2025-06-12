// Answer 0

#[test]
fn test_advance_mut_exact() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    unsafe {
        buffer.set_len(5);
        let remaining = buffer.capacity() - buffer.len(); // 5 remaining
        buffer.advance_mut(5); // Advance exactly the remaining bytes
    }
}

#[test]
#[should_panic]
fn test_advance_mut_overflow() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    unsafe {
        buffer.set_len(10);
        let remaining = buffer.capacity() - buffer.len(); // 0 remaining
        buffer.advance_mut(1); // Attempt to advance beyond remaining
    }
}

#[test]
fn test_advance_mut_no_advancement() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    unsafe {
        buffer.set_len(5);
        buffer.advance_mut(0); // Advance 0 bytes
    }
}

#[test]
fn test_advance_mut_partial() {
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    unsafe {
        buffer.set_len(3);
        let remaining = buffer.capacity() - buffer.len(); // 7 remaining
        buffer.advance_mut(7); // Advance exactly the remaining bytes
    }
}

