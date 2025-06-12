// Answer 0

#[derive(Debug)]
struct MockBuf {
    data: Vec<u8>,
    position: usize,
}

impl MockBuf {
    fn remaining(&self) -> usize {
        self.data.len() - self.position
    }
}

#[test]
fn test_remaining_with_data() {
    let buf = MockBuf {
        data: vec![1, 2, 3, 4, 5],
        position: 2,
    };
    assert_eq!(buf.remaining(), 3);
}

#[test]
fn test_remaining_without_data() {
    let buf = MockBuf {
        data: vec![],
        position: 0,
    };
    assert_eq!(buf.remaining(), 0);
}

#[test]
fn test_remaining_at_end() {
    let buf = MockBuf {
        data: vec![1, 2, 3],
        position: 3,
    };
    assert_eq!(buf.remaining(), 0);
}

#[test]
fn test_remaining_with_position_less_than_data_length() {
    let buf = MockBuf {
        data: vec![1, 2],
        position: 1,
    };
    assert_eq!(buf.remaining(), 1);
}

#[test]
fn test_remaining_with_position_greater_than_data_length() {
    let buf = MockBuf {
        data: vec![1],
        position: 2,
    };
    assert_eq!(buf.remaining(), 0);
}

