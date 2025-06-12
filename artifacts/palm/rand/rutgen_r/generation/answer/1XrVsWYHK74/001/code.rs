// Answer 0

#[derive(Default)]
struct Buffer {
    index: usize,
}

impl Buffer {
    pub fn index(&self) -> usize {
        self.index
    }
}

#[test]
fn test_index_zero() {
    let buffer = Buffer::default();
    assert_eq!(buffer.index(), 0);
}

#[test]
fn test_index_non_zero() {
    let mut buffer = Buffer::default();
    buffer.index = 5; // setting index to a non-zero value
    assert_eq!(buffer.index(), 5);
}

#[test]
fn test_index_large_value() {
    let mut buffer = Buffer::default();
    buffer.index = usize::MAX; // testing with maximum usize value
    assert_eq!(buffer.index(), usize::MAX);
}

