// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct TestSlice<'a> {
    slice: &'a [u8],
}

impl TestSlice<'_> {
    fn position_of_index(&self, i: usize) -> Position {
        let start_of_line = match memchr::memrchr(b'\n', &self.slice[..i]) {
            Some(position) => position + 1,
            None => 0,
        };
        Position {
            line: 1 + memchr::memchr_iter(b'\n', &self.slice[..start_of_line]).count(),
            column: i - start_of_line,
        }
    }
}

#[test]
fn test_position_of_index_empty() {
    let test_slice = TestSlice { slice: b"" };
    let position = test_slice.position_of_index(0);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

#[test]
fn test_position_of_index_single_line() {
    let test_slice = TestSlice { slice: b"Hello, World!" };
    let position = test_slice.position_of_index(5);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 5);
}

#[test]
fn test_position_of_index_multiple_lines() {
    let test_slice = TestSlice { slice: b"Hello\nWorld\n!" };
    let position = test_slice.position_of_index(8);
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 0);
}

#[test]
fn test_position_of_index_at_start_of_line() {
    let test_slice = TestSlice { slice: b"Line1\nLine2\n" };
    let position = test_slice.position_of_index(6);
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 0);
}

#[test]
fn test_position_of_index_first_character() {
    let test_slice = TestSlice { slice: b"Line\nAnother line\n" };
    let position = test_slice.position_of_index(0);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

#[test]
#[should_panic]
fn test_position_of_index_out_of_bounds() {
    let test_slice = TestSlice { slice: b"Line1\nLine2\n" };
    let _position = test_slice.position_of_index(15); // Out of bounds
}

