// Answer 0

#[derive(Default)]
struct MockPeeker {
    data: Vec<u8>,
    index: usize,
}

impl MockPeeker {
    fn new(data: Vec<u8>) -> Self {
        Self { data, index: 0 }
    }

    fn peek(&mut self) -> Result<u8, &'static str> {
        if self.index < self.data.len() {
            Ok(self.data[self.index])
        } else {
            Err("Out of bounds")
        }
    }
}

fn peek_or_null(peeker: &mut MockPeeker) -> Result<u8, &'static str> {
    Ok(peeker.peek().unwrap_or_else(|_| b'\x00'))
}

#[test]
fn test_peek_or_null_with_valid_data() {
    let mut peeker = MockPeeker::new(vec![1, 2, 3]);
    assert_eq!(peek_or_null(&mut peeker).unwrap(), 1);
}

#[test]
fn test_peek_or_null_with_empty_data() {
    let mut peeker = MockPeeker::new(vec![]);
    assert_eq!(peek_or_null(&mut peeker).unwrap(), b'\x00');
}

#[test]
fn test_peek_or_null_at_boundaries() {
    let mut peeker = MockPeeker::new(vec![b'A']);
    assert_eq!(peek_or_null(&mut peeker).unwrap(), b'A');
    
    // Move to invalid index to trigger panic condition
    peeker.index += 1;
    assert_eq!(peek_or_null(&mut peeker).unwrap(), b'\x00');
}

