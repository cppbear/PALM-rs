// Answer 0

#[test]
fn test_next_utf8_none_case() {
    let text: Vec<u8> = Vec::new(); // An empty vector simulates the None case
    let index = 0;
    let result = next_utf8(&text, index);
    assert_eq!(result, index + 1); // Expected to return 1
}

#[test]
fn test_next_utf8_out_of_bounds() {
    let text = vec![b'a', b'b', b'c']; // Valid UTF-8 sequence
    let index = text.len(); // Index is equal to the length of the array
    let result = next_utf8(&text, index);
    assert_eq!(result, index + 1); // Expected to return 4, since index is out of bounds
}

