// Answer 0

#[test]
fn test_write_str_success_exact_fit() {
    struct MockBytesMut {
        data: Vec<u8>,
        capacity: usize,
    }

    impl MockBytesMut {
        fn new(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }
    }

    let mut buf = MockBytesMut::new(5);
    let input = "hello";
    assert_eq!(buf.remaining_mut(), 5);
    let result = buf.write_str(input);
    assert!(result.is_ok());
    assert_eq!(buf.data, input.as_bytes());
} 

#[test]
fn test_write_str_success_zero_length() {
    struct MockBytesMut {
        data: Vec<u8>,
        capacity: usize,
    }

    impl MockBytesMut {
        fn new(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }
    }

    let mut buf = MockBytesMut::new(0);
    let input = "";
    assert_eq!(buf.remaining_mut(), 0);
    let result = buf.write_str(input);
    assert!(result.is_ok());
    assert_eq!(buf.data, input.as_bytes());
}

