// Answer 0

#[derive(Debug)]
struct Position {
    byte_index: usize,
}

struct SliceReader {
    slice: Vec<u8>,
    index: usize,
}

impl SliceReader {
    fn position_of_index(&self, index: usize) -> Position {
        Position { byte_index: index }
    }

    fn peek_position(&self) -> Position {
        self.position_of_index(std::cmp::min(self.slice.len(), self.index + 1))
    }
}

#[test]
fn test_peek_position_empty_slice() {
    let reader = SliceReader { slice: vec![], index: 0 };
    let position = reader.peek_position();
    assert_eq!(position.byte_index, 0);
}

#[test]
fn test_peek_position_single_byte() {
    let reader = SliceReader { slice: vec![10], index: 0 };
    let position = reader.peek_position();
    assert_eq!(position.byte_index, 1);
}

#[test]
fn test_peek_position_multiple_bytes() {
    let reader = SliceReader { slice: vec![10, 20, 30], index: 1 };
    let position = reader.peek_position();
    assert_eq!(position.byte_index, 2);
}

#[test]
fn test_peek_position_at_end() {
    let reader = SliceReader { slice: vec![10, 20, 30], index: 2 };
    let position = reader.peek_position();
    assert_eq!(position.byte_index, 3);
}

#[test]
fn test_peek_position_out_of_bounds() {
    let reader = SliceReader { slice: vec![10, 20, 30], index: 3 };
    let position = reader.peek_position();
    assert_eq!(position.byte_index, 3);
}

#[test]
fn test_peek_position_large_index() {
    let reader = SliceReader { slice: vec![10, 20, 30], index: 1000 };
    let position = reader.peek_position();
    assert_eq!(position.byte_index, 3);
}

