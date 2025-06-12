// Answer 0

#[test]
fn test_read_from_delegate_success() {
    use std::io::Cursor;
    use std::io::{self, Read};

    struct DummyReader {
        cursor: Cursor<Vec<u8>>,
    }

    impl Read for DummyReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.cursor.read(buf)
        }
    }

    const BUF_SIZE: usize = 10;

    struct TestStruct {
        b64_buffer: [u8; BUF_SIZE],
        b64_offset: usize,
        b64_len: usize,
        inner: DummyReader,
    }

    impl TestStruct {
        fn new(data: Vec<u8>) -> Self {
            let cursor = Cursor::new(data);
            Self {
                b64_buffer: [0; BUF_SIZE],
                b64_offset: 0,
                b64_len: 0,
                inner: DummyReader { cursor },
            }
        }

        fn read_from_delegate(&mut self) -> io::Result<usize> {
            debug_assert!(self.b64_offset + self.b64_len < BUF_SIZE);
            let read = self
                .inner
                .read(&mut self.b64_buffer[self.b64_offset + self.b64_len..])?;
            self.b64_len += read;
            debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
            Ok(read)
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3, 4, 5]);
    let bytes_read = test_struct.read_from_delegate().unwrap();
    assert_eq!(bytes_read, 5);
    assert_eq!(test_struct.b64_len, 5);
}

#[test]
#[should_panic]
fn test_read_from_delegate_no_space() {
    use std::io::Cursor;
    use std::io::{self, Read};

    struct DummyReader {
        cursor: Cursor<Vec<u8>>,
    }

    impl Read for DummyReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.cursor.read(buf)
        }
    }

    const BUF_SIZE: usize = 5;

    struct TestStruct {
        b64_buffer: [u8; BUF_SIZE],
        b64_offset: usize,
        b64_len: usize,
        inner: DummyReader,
    }

    impl TestStruct {
        fn new(data: Vec<u8>) -> Self {
            let cursor = Cursor::new(data);
            Self {
                b64_buffer: [0; BUF_SIZE],
                b64_offset: 0,
                b64_len: BUF_SIZE, // Setting len to BUF_SIZE to trigger panic
                inner: DummyReader { cursor },
            }
        }

        fn read_from_delegate(&mut self) -> io::Result<usize> {
            debug_assert!(self.b64_offset + self.b64_len < BUF_SIZE);
            let read = self
                .inner
                .read(&mut self.b64_buffer[self.b64_offset + self.b64_len..])?;
            self.b64_len += read;
            debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
            Ok(read)
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3]);
    let _ = test_struct.read_from_delegate(); // This should panic
}

