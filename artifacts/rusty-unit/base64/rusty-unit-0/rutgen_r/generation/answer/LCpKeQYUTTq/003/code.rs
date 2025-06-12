// Answer 0

#[test]
fn test_read_from_delegate_success() {
    use std::io::{self, Cursor};

    const BUF_SIZE: usize = 1024;

    struct InnerReader {
        data: Cursor<Vec<u8>>,
    }

    impl InnerReader {
        fn new(data: Vec<u8>) -> Self {
            InnerReader {
                data: Cursor::new(data),
            }
        }
    }

    impl std::io::Read for InnerReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.data.read(buf)
        }
    }

    struct Decoder {
        inner: InnerReader,
        b64_buffer: [u8; BUF_SIZE],
        b64_offset: usize,
        b64_len: usize,
    }

    impl Decoder {
        fn new(inner: InnerReader) -> Self {
            Decoder {
                inner,
                b64_buffer: [0; BUF_SIZE],
                b64_offset: 0,
                b64_len: 0,
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

    let data = vec![1, 2, 3, 4, 5];
    let inner_reader = InnerReader::new(data);
    let mut decoder = Decoder::new(inner_reader);
    decoder.b64_len = 5; // Fill the buffer with 5 bytes of valid data
    let result = decoder.read_from_delegate();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0); // No new bytes should be read since buffer is full
}

#[test]
#[should_panic]
fn test_read_from_delegate_panic_beyond_buf_size() {
    use std::io::{self, Cursor};

    const BUF_SIZE: usize = 1024;

    struct InnerReader {
        data: Cursor<Vec<u8>>,
    }

    impl InnerReader {
        fn new(data: Vec<u8>) -> Self {
            InnerReader {
                data: Cursor::new(data),
            }
        }
    }

    impl std::io::Read for InnerReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.data.read(buf)
        }
    }

    struct Decoder {
        inner: InnerReader,
        b64_buffer: [u8; BUF_SIZE],
        b64_offset: usize,
        b64_len: usize,
    }

    impl Decoder {
        fn new(inner: InnerReader) -> Self {
            Decoder {
                inner,
                b64_buffer: [0; BUF_SIZE],
                b64_offset: 0,
                b64_len: 0,
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

    let data = vec![1, 2, 3, 4, 5];
    let inner_reader = InnerReader::new(data);
    let mut decoder = Decoder::new(inner_reader);
    decoder.b64_offset = BUF_SIZE; // Offset is set to BUF_SIZE, should panic
    let _ = decoder.read_from_delegate();
}

#[test]
fn test_read_from_delegate_partial_read() {
    use std::io::{self, Cursor};

    const BUF_SIZE: usize = 1024;

    struct InnerReader {
        data: Cursor<Vec<u8>>,
    }

    impl InnerReader {
        fn new(data: Vec<u8>) -> Self {
            InnerReader {
                data: Cursor::new(data),
            }
        }
    }

    impl std::io::Read for InnerReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.data.read(buf)
        }
    }

    struct Decoder {
        inner: InnerReader,
        b64_buffer: [u8; BUF_SIZE],
        b64_offset: usize,
        b64_len: usize,
    }

    impl Decoder {
        fn new(inner: InnerReader) -> Self {
            Decoder {
                inner,
                b64_buffer: [0; BUF_SIZE],
                b64_offset: 0,
                b64_len: 0,
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
    
    let data = vec![1, 2, 3]; // Less than the BUF_SIZE
    let inner_reader = InnerReader::new(data);
    let mut decoder = Decoder::new(inner_reader);
    decoder.b64_len = 0; // Start with empty buffer
    let result = decoder.read_from_delegate();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3); // Expect to read all 3 bytes
}

