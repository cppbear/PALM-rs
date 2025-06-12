// Answer 0

#[test]
fn test_read_from_delegate_success() {
    use std::io::{self, Cursor};

    struct TestStruct {
        b64_offset: usize,
        b64_len: usize,
        b64_buffer: [u8; 10],
        inner: Cursor<Vec<u8>>,
    }

    impl TestStruct {
        fn new(inner_data: Vec<u8>) -> Self {
            Self {
                b64_offset: 0,
                b64_len: 0,
                b64_buffer: [0; 10],
                inner: Cursor::new(inner_data),
            }
        }

        fn read_from_delegate(&mut self) -> io::Result<usize> {
            debug_assert!(self.b64_offset + self.b64_len < 10);

            let read = self.inner.read(&mut self.b64_buffer[self.b64_offset + self.b64_len..])?;
            self.b64_len += read;

            debug_assert!(self.b64_offset + self.b64_len <= 10);

            Ok(read)
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3, 4, 5]);
    let result = test_struct.read_from_delegate().unwrap();
    assert_eq!(result, 5);
    assert_eq!(test_struct.b64_len, 5);
}

#[test]
#[should_panic]
fn test_read_from_delegate_exceeding_buffer_size() {
    use std::io::{self, Cursor};

    struct TestStruct {
        b64_offset: usize,
        b64_len: usize,
        b64_buffer: [u8; 10],
        inner: Cursor<Vec<u8>>,
    }

    impl TestStruct {
        fn new(inner_data: Vec<u8>) -> Self {
            Self {
                b64_offset: 0,
                b64_len: 10, // setting length to BUF_SIZE to cause panic
                b64_buffer: [0; 10],
                inner: Cursor::new(inner_data),
            }
        }

        fn read_from_delegate(&mut self) -> io::Result<usize> {
            debug_assert!(self.b64_offset + self.b64_len < 10);

            let read = self.inner.read(&mut self.b64_buffer[self.b64_offset + self.b64_len..])?;
            self.b64_len += read;

            debug_assert!(self.b64_offset + self.b64_len <= 10);

            Ok(read)
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3, 4, 5]);
    let _result = test_struct.read_from_delegate().unwrap();
}

