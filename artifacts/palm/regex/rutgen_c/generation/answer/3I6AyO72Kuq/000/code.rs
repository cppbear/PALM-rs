// Answer 0

#[test]
fn test_deref() {
    let input_data: &[u8] = b"Hello, world!";
    let char_input = CharInput(input_data);
    let dereferenced = char_input.deref();
    assert_eq!(dereferenced, input_data);
}

