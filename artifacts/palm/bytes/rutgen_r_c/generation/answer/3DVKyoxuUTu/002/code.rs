// Answer 0

#[test]
#[should_panic(expected = "cannot advance past `remaining`")]
fn test_advance_panic() {
    let mut bytes = Bytes::new(); // Initializing with empty Bytes
    bytes.advance(1); // This should cause a panic since len() is 0
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`")]
fn test_advance_panic_exact_length() {
    let data = b"test data"; // Testing with a string to have a defined length
    let mut bytes = Bytes::from_static(data); // Initializing with a static byte array
    bytes.advance(5); // This should be fine for moving within bounds
    bytes.advance(1); // Now here it should panic since len() is now 5 (after previous advance)
}

