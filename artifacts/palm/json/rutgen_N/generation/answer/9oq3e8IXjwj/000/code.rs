// Answer 0

#[derive(Debug)]
struct SliceReader<'a> {
    slice: &'a [u8],
    index: usize,
}

impl<'a> SliceReader<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { slice, index: 0 }
    }

    fn skip_to_escape_slow(&mut self) {
        while self.index < self.slice.len() && !is_escape(self.slice[self.index], true) {
            self.index += 1;
        }
    }
}

fn is_escape(byte: u8, _: bool) -> bool {
    byte == b'\\' // Example escape character
}

#[test]
fn test_skip_to_escape_slow_no_escape() {
    let mut reader = SliceReader::new(b"hello, world!");
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 13); // End of the slice
}

#[test]
fn test_skip_to_escape_slow_with_escape() {
    let mut reader = SliceReader::new(b"hello, \\world!");
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 7); // Index of escape character
}

#[test]
fn test_skip_to_escape_slow_empty_slice() {
    let mut reader = SliceReader::new(b"");
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 0); // Index should remain at 0
}

#[test]
fn test_skip_to_escape_slow_edge_case() {
    let mut reader = SliceReader::new(b"abc\\xyz");
    reader.skip_to_escape_slow();
    assert_eq!(reader.index, 3); // Index should stop right before escape
}

