// Answer 0

#[derive(Default)]
struct ResultBuffer {
    index: usize,
}

impl ResultBuffer {
    pub fn index(&self) -> usize {
        self.index
    }
}

#[test]
fn test_index_zero() {
    let buffer = ResultBuffer::default();
    assert_eq!(buffer.index(), 0);
}

#[test]
fn test_index_non_zero() {
    let mut buffer = ResultBuffer { index: 5 };
    assert_eq!(buffer.index(), 5);
}

#[test]
fn test_index_large_value() {
    let mut buffer = ResultBuffer { index: usize::MAX };
    assert_eq!(buffer.index(), usize::MAX);
}

#[test]
fn test_index_after_modification() {
    let mut buffer = ResultBuffer { index: 10 };
    buffer.index = 20; // Direct modification for testing purposes
    assert_eq!(buffer.index(), 20);
}

