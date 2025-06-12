// Answer 0

#[test]
fn test_advance_overflow() {
    let mut cursor = std::io::Cursor::new(vec![0u8; 5]); // Initialize a buffer with 5 bytes
    cursor.set_position(5); // Set position at the end of the buffer
    let cnt = 6; // cnt > max_cnt (which is 0 when at position 5)
    let result = std::panic::catch_unwind(|| cursor.advance(cnt));
    assert!(result.is_err()); // Ensure it panics
}

#[test]
fn test_advance_large_count() {
    let mut cursor = std::io::Cursor::new(vec![0u8; 10]); // Initialize a buffer with 10 bytes
    cursor.set_position(10); // Set position at the end of the buffer
    let cnt = usize::MAX; // cnt > max_cnt
    let result = std::panic::catch_unwind(|| cursor.advance(cnt));
    assert!(result.is_err()); // Ensure it panics
}

#[test]
fn test_advance_exceeding_cnt() {
    let mut cursor = std::io::Cursor::new(vec![0u8; 8]); // Initialize a buffer with 8 bytes
    cursor.set_position(3); // Set position somewhere in the buffer
    let cnt = 6; // cnt > max_cnt (which is 5 when at position 3)
    let result = std::panic::catch_unwind(|| cursor.advance(cnt));
    assert!(result.is_err()); // Ensure it panics
}

#[test]
fn test_advance_exact_end() {
    let mut cursor = std::io::Cursor::new(vec![1u8; 5]); // Buffer with 5 bytes
    cursor.set_position(5); // Position at the end
    let cnt = 0; // Should not panic since advancing zero is allowed
    cursor.advance(cnt); // Just a call; does not panic
}

