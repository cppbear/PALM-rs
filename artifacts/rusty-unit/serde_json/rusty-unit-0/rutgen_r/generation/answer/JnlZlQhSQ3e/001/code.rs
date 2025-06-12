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
fn test_position_of_index_middle() {
    let slice = b"Hello\nworld\nthis is a test";
    let wrapper = SliceWrapper { slice };
    let pos = wrapper.position_of_index(10);
    assert_eq!(pos, Position { line: 2, column: 0 });
}

#[test]
fn test_position_of_index_start() {
    let slice = b"First line\nSecond line";
    let wrapper = SliceWrapper { slice };
    let pos = wrapper.position_of_index(0);
    assert_eq!(pos, Position { line: 1, column: 0 });
}

#[test]
fn test_position_of_index_end() {
    let slice = b"Line one\nLine two\nLine three";
    let wrapper = SliceWrapper { slice };
    let pos = wrapper.position_of_index(26);
    assert_eq!(pos, Position { line: 3, column: 0 });
}

#[test]
fn test_position_of_index_no_newline() {
    let slice = b"Single line";
    let wrapper = SliceWrapper { slice };
    let pos = wrapper.position_of_index(10);
    assert_eq!(pos, Position { line: 1, column: 10 });
}

#[test]
#[should_panic]
fn test_position_of_index_out_of_bounds() {
    let slice = b"Short line";
    let wrapper = SliceWrapper { slice };
    let _ = wrapper.position_of_index(20);
}

