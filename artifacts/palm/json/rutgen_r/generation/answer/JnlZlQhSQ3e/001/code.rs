// Answer 0

fn position_of_index(slice: &[u8], i: usize) -> Position {
    let start_of_line = match memchr::memrchr(b'\n', &slice[..i]) {
        Some(position) => position + 1,
        None => 0,
    };
    Position {
        line: 1 + memchr::memchr_iter(b'\n', &slice[..start_of_line]).count(),
        column: i - start_of_line,
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    line: usize,
    column: usize,
}

#[test]
fn test_position_of_index_with_single_line() {
    let text = b"Hello, World!";
    let expected = Position { line: 1, column: 5 };
    let result = position_of_index(text, 5);
    assert_eq!(result, expected);
}

#[test]
fn test_position_of_index_with_multiple_lines() {
    let text = b"Line one\nLine two\nLine three";
    let expected = Position { line: 2, column: 4 };
    let result = position_of_index(text, 12);
    assert_eq!(result, expected);
}

#[test]
fn test_position_of_index_with_newline_at_start() {
    let text = b"\nFirst line\nSecond line";
    let expected = Position { line: 1, column: 0 };
    let result = position_of_index(text, 0);
    assert_eq!(result, expected);
}

#[test]
fn test_position_of_index_with_newline_at_end() {
    let text = b"First line\nSecond line\n";
    let expected = Position { line: 2, column: 0 };
    let result = position_of_index(text, 15);
    assert_eq!(result, expected);
}

#[test]
fn test_position_of_index_with_no_newline() {
    let text = b"Single uninterrupted line.";
    let expected = Position { line: 1, column: 30 };
    let result = position_of_index(text, 30);
    assert_eq!(result, expected);
}

#[should_panic]
fn test_position_of_index_with_out_of_bounds_index() {
    let text = b"Short text";
    let _ = position_of_index(text, 12); // This should trigger a panic
}

