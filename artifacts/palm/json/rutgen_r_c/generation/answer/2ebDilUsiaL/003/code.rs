// Answer 0

#[test]
fn test_indent_zero_iterations() {
    use std::io::Cursor;

    let mut buf = Vec::new();
    let cursor = &mut Cursor::new(&mut buf);
    let result = indent(cursor, 0, b"indentation");
    assert_eq!(result, Ok(()));
    assert_eq!(buf.len(), 0);
}

#[test]
fn test_indent_one_iteration() {
    use std::io::Cursor;

    let mut buf = Vec::new();
    let cursor = &mut Cursor::new(&mut buf);
    let result = indent(cursor, 1, b"indentation");
    assert_eq!(result, Ok(()));
    assert_eq!(buf, b"indentation");
}

#[test]
fn test_indent_multiple_iterations() {
    use std::io::Cursor;

    let mut buf = Vec::new();
    let cursor = &mut Cursor::new(&mut buf);
    let result = indent(cursor, 3, b"indentation");
    assert_eq!(result, Ok(()));
    assert_eq!(buf, b"indentationindentationindentation");
}

