// Answer 0

#[test]
fn test_indent_with_zero_indent() {
    use std::io::Cursor;

    let mut output = Cursor::new(Vec::new());
    let input = b"Hello, World!";
    let result = indent(&mut output, 0, input);
    
    assert!(result.is_ok());
    assert_eq!(output.get_ref(), b"");
}

#[test]
fn test_indent_with_one_indent() {
    use std::io::Cursor;

    let mut output = Cursor::new(Vec::new());
    let input = b"Hello, World!";
    let result = indent(&mut output, 1, input);
    
    assert!(result.is_ok());
    assert_eq!(output.get_ref(), b"Hello, World!Hello, World!");
}

#[test]
fn test_indent_with_multiple_indent() {
    use std::io::Cursor;

    let mut output = Cursor::new(Vec::new());
    let input = b"Hello!";
    let result = indent(&mut output, 3, input);
    
    assert!(result.is_ok());
    assert_eq!(output.get_ref(), b"Hello!Hello!Hello!");
}

