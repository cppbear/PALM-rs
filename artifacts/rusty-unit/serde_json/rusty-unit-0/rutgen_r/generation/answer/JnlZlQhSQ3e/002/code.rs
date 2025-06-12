// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct SliceWrapper<'a> {
    slice: &'a [u8],
}

impl<'a> SliceWrapper<'a> {
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
fn test_position_of_index_no_newline() {
    let data = b"Hello, world!";
    let slice_wrapper = SliceWrapper { slice: data };
    let position = slice_wrapper.position_of_index(5);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 5);
}

#[test]
fn test_position_of_index_with_newline() {
    let data = b"Hello,\nworld!";
    let slice_wrapper = SliceWrapper { slice: data };
    let position = slice_wrapper.position_of_index(7);
    assert_eq!(position.line, 2);
    assert_eq!(position.column, 0);
}

#[test]
fn test_position_of_index_empty_slice() {
    let data: &[u8] = b"";
    let slice_wrapper = SliceWrapper { slice: data };
    let position = slice_wrapper.position_of_index(0);
    assert_eq!(position.line, 1);
    assert_eq!(position.column, 0);
}

#[test]
#[should_panic]
fn test_position_of_index_out_of_bounds() {
    let data = b"Test data";
    let slice_wrapper = SliceWrapper { slice: data };
    let _position = slice_wrapper.position_of_index(data.len() + 1); // This should panic
}

