// Answer 0

#[test]
fn test_flush_decoded_buf_success() {
    use std::io;
    use std::cmp;

    struct Decoder {
        decoded_chunk_buffer: Vec<u8>,
        decoded_len: usize,
        decoded_offset: usize,
    }

    const DECODED_CHUNK_SIZE: usize = 10;

    let mut decoder = Decoder {
        decoded_chunk_buffer: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        decoded_len: 10,
        decoded_offset: 0,
    };
    let mut buffer = [0u8; 5];

    let result = decoder.flush_decoded_buf(&mut buffer).unwrap();

    assert_eq!(result, 5);
    assert_eq!(&buffer[..result], &[1, 2, 3, 4, 5]);
    assert_eq!(decoder.decoded_offset, 5);
    assert_eq!(decoder.decoded_len, 5);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_no_space() {
    use std::io;

    struct Decoder {
        decoded_chunk_buffer: Vec<u8>,
        decoded_len: usize,
        decoded_offset: usize,
    }

    let mut decoder = Decoder {
        decoded_chunk_buffer: vec![1, 2, 3],
        decoded_len: 3,
        decoded_offset: 0,
    };
    let mut buffer = [0u8; 0];

    let _ = decoder.flush_decoded_buf(&mut buffer).unwrap();
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_buffer() {
    use std::io;

    struct Decoder {
        decoded_chunk_buffer: Vec<u8>,
        decoded_len: usize,
        decoded_offset: usize,
    }

    let mut decoder = Decoder {
        decoded_chunk_buffer: vec![1, 2, 3],
        decoded_len: 3,
        decoded_offset: 0,
    };
    let mut buffer = [0u8; 5];

    decoder.decoded_len = 0; // Simulate no data to write

    let _ = decoder.flush_decoded_buf(&mut buffer).unwrap();
}

