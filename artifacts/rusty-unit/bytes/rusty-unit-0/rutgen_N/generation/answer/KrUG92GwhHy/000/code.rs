// Answer 0

#[test]
fn test_write_str_success() {
    struct BytesMut {
        data: Vec<u8>,
        capacity: usize,
    }

    impl BytesMut {
        fn new(capacity: usize) -> Self {
            BytesMut {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        fn put_slice(&mut self, slice: &[u8]) {
            self.data.extend_from_slice(slice);
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            if self.remaining_mut() >= s.len() {
                self.put_slice(s.as_bytes());
                Ok(())
            } else {
                Err(std::fmt::Error)
            }
        }
    }

    let mut bytes_mut = BytesMut::new(10);
    let result = bytes_mut.write_str("Hello");
    assert!(result.is_ok());
    assert_eq!(bytes_mut.data, b"Hello");
}

#[test]
fn test_write_str_failure() {
    struct BytesMut {
        data: Vec<u8>,
        capacity: usize,
    }

    impl BytesMut {
        fn new(capacity: usize) -> Self {
            BytesMut {
                data: Vec::with_capacity(capacity),
                capacity,
            }
        }

        fn remaining_mut(&self) -> usize {
            self.capacity - self.data.len()
        }

        fn put_slice(&mut self, slice: &[u8]) {
            self.data.extend_from_slice(slice);
        }

        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            if self.remaining_mut() >= s.len() {
                self.put_slice(s.as_bytes());
                Ok(())
            } else {
                Err(std::fmt::Error)
            }
        }
    }

    let mut bytes_mut = BytesMut::new(5);
    let _ = bytes_mut.write_str("Hello");
    let result = bytes_mut.write_str("World");
    assert!(result.is_err());
    assert_eq!(bytes_mut.data, b"Hello");
}

