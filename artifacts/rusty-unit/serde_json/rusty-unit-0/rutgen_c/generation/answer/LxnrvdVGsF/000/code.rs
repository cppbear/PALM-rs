// Answer 0

#[test]
fn ignore_str_test_valid_string() {
    let input = b"hello, world!\"";
    let mut slice_read = SliceRead::new(input);
    let result = slice_read.ignore_str();
    assert!(result.is_ok());
    assert_eq!(slice_read.index, 15); // Index should move past string
}

#[test]
fn ignore_str_test_unescaped_control_character() {
    let input = b"hello, world!\x01"; // Contains control character
    let mut slice_read = SliceRead::new(input);
    let result = slice_read.ignore_str();
    
    assert!(result.is_err());
    if let Err(Error { err: box ErrorCode::ControlCharacterWhileParsingString }) = result {
        // Expected to hit this error
    } else {
        panic!("Expected control character error");
    }
}

#[test]
fn ignore_str_test_eof() {
    let input = b"hello, world!";
    let mut slice_read = SliceRead::new(input);
    slice_read.index = input.len(); // Set index at the end to simulate EOF
    let result = slice_read.ignore_str();
    
    assert!(result.is_err());
    if let Err(Error { err: box ErrorCode::EofWhileParsingString }) = result {
        // Expected to hit this error
    } else {
        panic!("Expected EOF error");
    }
}

