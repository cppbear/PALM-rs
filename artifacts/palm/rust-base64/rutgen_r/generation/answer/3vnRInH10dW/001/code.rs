// Answer 0

#[test]
fn test_read_u64_valid_input() {
    let input: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1]; // Should decode to 1
    assert_eq!(read_u64(&input), 1);
}

#[test]
fn test_read_u64_large_value() {
    let input: [u8; 8] = [255, 255, 255, 255, 255, 255, 255, 255]; // Should decode to 18,446,744,073,709,551,615
    assert_eq!(read_u64(&input), 18_446_744_073_709_551_615);
}

#[test]
fn test_read_u64_zero_input() {
    let input: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0]; // Should decode to 0
    assert_eq!(read_u64(&input), 0);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_read_u64_short_input() {
    let input: [u8; 7] = [0, 0, 0, 0, 0, 0, 0]; // Only 7 bytes, should panic
    let _ = read_u64(&input);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_read_u64_empty_input() {
    let input: [u8; 0] = []; // No bytes, should panic
    let _ = read_u64(&input);
}

