// Answer 0

#[test]
fn test_flush_decoded_buf_success_full_copy() {
    use std::io;
    use std::cmp;

    const DECODED_CHUNK_SIZE: usize = 64;

    struct Decoder {
        decoded_chunk_buffer: [u8; DECODED_CHUNK_SIZE],
        decoded_len: usize,
        decoded_offset: usize,
    }

    impl Decoder {
        fn new(data: &[u8], decoded_len: usize) -> Self {
            let mut buffer = [0u8; DECODED_CHUNK_SIZE];
            buffer[..data.len()].copy_from_slice(data);
            Self {
                decoded_chunk_buffer: buffer,
                decoded_len,
                decoded_offset: 0,
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

    let input_data = b"Example data to decode";
    let mut decoder = Decoder::new(input_data, input_data.len());

    let mut target_buffer = [0u8; 64];
    let result = decoder.flush_decoded_buf(&mut target_buffer[..]);

    assert_eq!(result, Ok(input_data.len()));
    assert_eq!(&target_buffer[..input_data.len()], input_data);
}

#[test]
fn test_flush_decoded_buf_partial_copy() {
    use std::io;
    use std::cmp;

    const DECODED_CHUNK_SIZE: usize = 64;

    struct Decoder {
        decoded_chunk_buffer: [u8; DECODED_CHUNK_SIZE],
        decoded_len: usize,
        decoded_offset: usize,
    }

    impl Decoder {
        fn new(data: &[u8], decoded_len: usize) -> Self {
            let mut buffer = [0u8; DECODED_CHUNK_SIZE];
            buffer[..data.len()].copy_from_slice(data);
            Self {
                decoded_chunk_buffer: buffer,
                decoded_len,
                decoded_offset: 0,
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

    let input_data = b"Another example";
    let mut decoder = Decoder::new(input_data, input_data.len());

    let mut target_buffer = [0u8; 10];
    let result = decoder.flush_decoded_buf(&mut target_buffer[..]);

    assert_eq!(result, Ok(target_buffer.len()));
    assert_eq!(&target_buffer[..input_data.len()], input_data);
}

