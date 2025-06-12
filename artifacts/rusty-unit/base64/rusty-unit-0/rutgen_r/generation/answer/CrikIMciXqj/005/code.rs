// Answer 0

#[test]
fn test_flush_decoded_buf_valid_case() {
    struct Decoder {
        decoded_len: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    impl Decoder {
        fn new(data: &[u8]) -> Self {
            Self {
                decoded_len: data.len(),
                decoded_offset: 0,
                decoded_chunk_buffer: data.to_vec(),
            }
        }

        fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            debug_assert!(self.decoded_len > 0);
            debug_assert!(!buf.is_empty());

            let copy_len = cmp::min(self.decoded_len, buf.len());
            debug_assert!(copy_len > 0);
            debug_assert!(copy_len <= self.decoded_len);

            buf[..copy_len].copy_from_slice(
                &self.decoded_chunk_buffer[self.decoded_offset..self.decoded_offset + copy_len],
            );

            self.decoded_offset += copy_len;
            self.decoded_len -= copy_len;

            debug_assert!(self.decoded_len < DECODED_CHUNK_SIZE);

            Ok(copy_len)
        }
    }

    const DECODED_CHUNK_SIZE: usize = 8;

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut decoder = Decoder::new(&data);
    let mut buffer = [0; 4];

    let result = decoder.flush_decoded_buf(&mut buffer).unwrap();

    assert_eq!(result, 4);
    assert_eq!(&buffer[..result], &[1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_buf_should_panic() {
    struct Decoder {
        decoded_len: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    impl Decoder {
        fn new(data: &[u8]) -> Self {
            Self {
                decoded_len: data.len(),
                decoded_offset: 0,
                decoded_chunk_buffer: data.to_vec(),
            }
        }

        fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            debug_assert!(self.decoded_len > 0);
            debug_assert!(!buf.is_empty());

            let copy_len = cmp::min(self.decoded_len, buf.len());
            debug_assert!(copy_len > 0);
            debug_assert!(copy_len <= self.decoded_len);

            buf[..copy_len].copy_from_slice(
                &self.decoded_chunk_buffer[self.decoded_offset..self.decoded_offset + copy_len],
            );

            self.decoded_offset += copy_len;
            self.decoded_len -= copy_len;

            debug_assert!(self.decoded_len < DECODED_CHUNK_SIZE);

            Ok(copy_len)
        }
    }

    const DECODED_CHUNK_SIZE: usize = 8;

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut decoder = Decoder::new(&data);
    let mut buffer: &[u8] = &[];

    decoder.flush_decoded_buf(&mut buffer).unwrap();
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_zero_copy_len() {
    struct Decoder {
        decoded_len: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    impl Decoder {
        fn new(data: &[u8]) -> Self {
            Self {
                decoded_len: data.len(),
                decoded_offset: 0,
                decoded_chunk_buffer: data.to_vec(),
            }
        }

        fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            debug_assert!(self.decoded_len > 0);
            debug_assert!(!buf.is_empty());

            let copy_len = cmp::min(self.decoded_len, buf.len());
            debug_assert!(copy_len > 0);
            debug_assert!(copy_len <= self.decoded_len); 

            buf[..copy_len].copy_from_slice(
                &self.decoded_chunk_buffer[self.decoded_offset..self.decoded_offset + copy_len],
            );

            self.decoded_offset += copy_len;
            self.decoded_len -= copy_len;

            debug_assert!(self.decoded_len < DECODED_CHUNK_SIZE);

            Ok(copy_len)
        }
    }

    const DECODED_CHUNK_SIZE: usize = 8;

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut decoder = Decoder::new(&data);
    let mut buffer = [0; 0]; // buffer with zero length

    decoder.flush_decoded_buf(&mut buffer).unwrap();
}

