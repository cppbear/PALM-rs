// Answer 0

#[derive(Debug)]
struct PeekableSlice {
    slice: Vec<u8>,
    index: usize,
}

impl PeekableSlice {
    fn new(slice: Vec<u8>, index: usize) -> Self {
        PeekableSlice { slice, index }
    }

    fn peek(&mut self) -> Result<Option<u8>, &'static str> {
        Ok(if self.index < self.slice.len() {
            Some(self.slice[self.index])
        } else {
            None
        })
    }
}

#[test]
fn test_peek_index_at_slice_length() {
    let slice = vec![1, 2, 3];
    let mut peekable = PeekableSlice::new(slice, 3);
    let result = peekable.peek();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_peek_empty_slice() {
    let slice: Vec<u8> = vec![];
    let mut peekable = PeekableSlice::new(slice, 0);
    let result = peekable.peek();
    assert_eq!(result, Ok(None));
}

