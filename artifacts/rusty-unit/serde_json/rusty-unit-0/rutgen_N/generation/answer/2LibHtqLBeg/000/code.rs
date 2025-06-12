// Answer 0

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}

struct SliceReader {
    slice: Vec<u8>,
    index: usize,
}

impl SliceReader {
    fn new(slice: Vec<u8>) -> Self {
        Self { slice, index: 0 }
    }

    fn position_of_index(&self, index: usize) -> Position {
        // This is a simplistic implementation for demonstration.
        Position {
            line: index / 10,
            column: index % 10,
        }
    }

    fn peek_position(&self) -> Position {
        self.position_of_index(std::cmp::min(self.slice.len(), self.index + 1))
    }
}

#[test]
fn test_peek_position_empty_slice() {
    let reader = SliceReader::new(Vec::new());
    let position = reader.peek_position();
    assert_eq!(position, Position { line: 0, column: 0 });
}

#[test]
fn test_peek_position_single_element() {
    let reader = SliceReader::new(vec![1]);
    let position = reader.peek_position();
    assert_eq!(position, Position { line: 0, column: 1 });
}

#[test]
fn test_peek_position_multiple_elements() {
    let reader = SliceReader::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    reader.index = 9; // Set index to the last element
    let position = reader.peek_position();
    assert_eq!(position, Position { line: 1, column: 0 });
}

#[test]
fn test_peek_position_index_out_of_bounds() {
    let reader = SliceReader::new(vec![1, 2, 3]);
    reader.index = 3; // Set index beyond the length of the slice
    let position = reader.peek_position();
    assert_eq!(position, Position { line: 0, column: 3 });
}

