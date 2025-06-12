// Answer 0

#[test]
fn test_indent_zero_times() {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    let result = indent(&mut buffer, 0, b"test");
    assert!(result.is_ok());
    assert!(buffer.get_ref().is_empty());
}

#[test]
fn test_indent_one_time() {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    let result = indent(&mut buffer, 1, b"test");
    assert!(result.is_ok());
    assert_eq!(buffer.get_ref().as_slice(), b"test");
}

#[test]
fn test_indent_multiple_times() {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    let result = indent(&mut buffer, 3, b"line\n");
    assert!(result.is_ok());
    assert_eq!(buffer.get_ref().as_slice(), b"line\nline\nline\n");
} 

#[test]
fn test_indent_empty_string() {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    let result = indent(&mut buffer, 2, b"");
    assert!(result.is_ok());
    assert!(buffer.get_ref().is_empty());
} 

#[test]
fn test_indent_large_n() {
    use std::io::Cursor;
    let mut buffer = Cursor::new(Vec::new());
    let result = indent(&mut buffer, 1000, b"line\n");
    assert!(result.is_ok());
    let expected_output = b"line\n".repeat(1000);
    assert_eq!(buffer.get_ref().as_slice(), expected_output.as_slice());
}

