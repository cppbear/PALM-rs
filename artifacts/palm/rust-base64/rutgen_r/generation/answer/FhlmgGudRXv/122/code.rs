// Answer 0

#[test]
fn test_read_empty_buf() {
    let mut decoder = Decoder {
        b64_offset: 0,
        b64_len: 0,
        decoded_len: 0,
        decoded_offset: 0,
        b64_buffer: [0; BUF_SIZE],
        decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
    };
    let mut buf = [];
    assert_eq!(decoder.read(&mut buf).unwrap(), 0);
}

// Additional test to cover edge cases regarding `self.b64_len == BASE64_CHUNK_SIZE`
#[test]
fn test_read_full_base64_chunk() {
    let mut decoder = Decoder {
        b64_offset: 0,
        b64_len: BASE64_CHUNK_SIZE,
        decoded_len: 2,
        decoded_offset: 0,
        b64_buffer: [b'A'; BUF_SIZE],
        decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
    };
    let mut buf = [0; DECODED_CHUNK_SIZE];
    assert_eq!(decoder.read(&mut buf).unwrap(), DECODED_CHUNK_SIZE);
}

// Test where `b64_offset` equals `BUF_SIZE` preventing copy_within.
#[test]
fn test_read_b64_offset_equals_buf_size() {
    let mut decoder = Decoder {
        b64_offset: BUF_SIZE,
        b64_len: BUF_SIZE,
        decoded_len: 2,
        decoded_offset: 0,
        b64_buffer: [0; BUF_SIZE],
        decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
    };
    let mut buf = [0; DECODED_CHUNK_SIZE];
    assert_eq!(decoder.read(&mut buf).unwrap(), DECODED_CHUNK_SIZE);
}

// This test case ensures decoding when the length of decoded is full but we have leftovers to flush
#[test]
fn test_read_with_leftover_decoded_bytes() {
    let mut decoder = Decoder {
        b64_offset: 1,
        b64_len: BUF_SIZE,
        decoded_len: 3,
        decoded_offset: 0,
        b64_buffer: [b'A', b'B', b'C', 0, 0],
        decoded_chunk_buffer: [b'D'; DECODED_CHUNK_SIZE],
    };
    let mut buf = [0; DECODED_CHUNK_SIZE];
    assert_eq!(decoder.read(&mut buf).unwrap(), DECODED_CHUNK_SIZE);
}

// Test to cover a panic condition on copy_within
#[should_panic(expected = "index out of bounds: the length is 0 but the index is 0")]
#[test]
fn test_read_buf_len_zero() {
    let mut decoder = Decoder {
        b64_offset: 2,
        b64_len: 3,
        decoded_len: 1,
        decoded_offset: 0,
        b64_buffer: [b'E', b'F', b'G', 0, 0],
        decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
    };
    let mut buf = [];
    decoder.read(&mut buf).unwrap();
}

