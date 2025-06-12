// Answer 0

#[derive(Default)]
struct Decoder {
    decoded_chunk_buffer: Vec<u8>,
    decoded_offset: usize,
    decoded_len: usize,
}

impl Decoder {
    fn new(decoded_chunk_buffer: Vec<u8>, decoded_len: usize) -> Self {
        Self {
            decoded_chunk_buffer,
            decoded_offset: 0,
            decoded_len,
        }
    }

    fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        debug_assert!(self.decoded_len > 0);
        debug_assert!(!buf.is_empty());

        let copy_len = std::cmp::min(self.decoded_len, buf.len());
        debug_assert!(copy_len > 0);
        debug_assert!(copy_len <= self.decoded_len);

        buf[..copy_len].copy_from_slice(
            &self.decoded_chunk_buffer[self.decoded_offset..self.decoded_offset + copy_len],
        );

        self.decoded_offset += copy_len;
        self.decoded_len -= copy_len;

        debug_assert!(self.decoded_len < 16);

        Ok(copy_len)
    }
}

#[test]
fn test_flush_decoded_buf_basic() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4, 5], 5);
    let mut buf = [0u8; 3];
    let result = decoder.flush_decoded_buf(&mut buf);
    assert_eq!(result.unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);
}

#[test]
fn test_flush_decoded_buf_exact_fit() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4], 4);
    let mut buf = [0u8; 4];
    let result = decoder.flush_decoded_buf(&mut buf);
    assert_eq!(result.unwrap(), 4);
    assert_eq!(buf, [1, 2, 3, 4]);
}

#[test]
fn test_flush_decoded_buf_partial() {
    let mut decoder = Decoder::new(vec![1, 2, 3, 4, 5, 6], 6);
    let mut buf = [0u8; 2];
    let result = decoder.flush_decoded_buf(&mut buf);
    assert_eq!(result.unwrap(), 2);
    assert_eq!(buf, [1, 2]);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_buf() {
    let mut decoder = Decoder::new(vec![1, 2, 3], 3);
    let mut buf: [u8; 0] = [];
    decoder.flush_decoded_buf(&mut buf).unwrap();
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_no_data_left() {
    let mut decoder = Decoder::new(vec![1], 1);
    let mut buf = [0u8; 1];
    decoder.flush_decoded_buf(&mut buf).unwrap();
    decoder.flush_decoded_buf(&mut buf).unwrap(); // should panic, no data left to copy
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_exceed_copy_len() {
    let mut decoder = Decoder::new(vec![1, 2], 2);
    let mut buf = [0u8; 1];
    decoder.flush_decoded_buf(&mut buf).unwrap(); 
    let result = decoder.flush_decoded_buf(&mut buf); // should panic since copy_len should not exceed
}

