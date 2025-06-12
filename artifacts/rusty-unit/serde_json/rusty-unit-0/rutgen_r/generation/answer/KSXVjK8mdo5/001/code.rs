// Answer 0

#[derive(Default)]
struct Delegate {
    offset: usize,
}

impl Delegate {
    fn byte_offset(&self) -> usize {
        self.offset
    }
}

struct DataReader {
    delegate: Delegate,
}

impl DataReader {
    fn new(offset: usize) -> Self {
        Self {
            delegate: Delegate { offset },
        }
    }

    fn byte_offset(&self) -> usize {
        self.delegate.byte_offset()
    }
}

#[test]
fn test_byte_offset_with_zero_offset() {
    let reader = DataReader::new(0);
    assert_eq!(reader.byte_offset(), 0);
}

#[test]
fn test_byte_offset_with_positive_offset() {
    let reader = DataReader::new(100);
    assert_eq!(reader.byte_offset(), 100);
}

#[test]
fn test_byte_offset_with_large_offset() {
    let reader = DataReader::new(usize::MAX);
    assert_eq!(reader.byte_offset(), usize::MAX);
}

