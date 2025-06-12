// Answer 0

#[derive(Debug)]
struct PeekableSlice {
    slice: Vec<u8>,
    index: usize,
}

impl PeekableSlice {
    fn peek(&mut self) -> Result<Option<u8>, &'static str> {
        Ok(if self.index < self.slice.len() {
            Some(self.slice[self.index])
        } else {
            None
        })
    }
}

#[test]
fn test_peek_index_equals_slice_length() {
    let mut peekable = PeekableSlice {
        slice: vec![1, 2, 3],
        index: 3,
    };
    let result = peekable.peek();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_peek_empty_slice() {
    let mut peekable = PeekableSlice {
        slice: vec![],
        index: 0,
    };
    let result = peekable.peek();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_peek_beyond_slice_length() {
    let mut peekable = PeekableSlice {
        slice: vec![5, 10, 15],
        index: 3,
    };
    let result = peekable.peek();
    assert_eq!(result, Ok(None));
}

