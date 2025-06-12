// Answer 0

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_buffer() {
    struct Decoder {
        decoded_len: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    let mut decoder = Decoder {
        decoded_len: 5,
        decoded_offset: 0,
        decoded_chunk_buffer: vec![1, 2, 3, 4, 5],
    };

    let mut buf: [u8; 0] = [];

    let result = decoder.flush_decoded_buf(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_flush_decoded_buf_partial_copy() {
    struct Decoder {
        decoded_len: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    let mut decoder = Decoder {
        decoded_len: 5,
        decoded_offset: 0,
        decoded_chunk_buffer: vec![1, 2, 3, 4, 5],
    };

    let mut buf: [u8; 3] = [0; 3];

    let result = decoder.flush_decoded_buf(&mut buf).unwrap();
    assert_eq!(result, 3);
    assert_eq!(buf, [1, 2, 3]);
    assert_eq!(decoder.decoded_offset, 3);
    assert_eq!(decoder.decoded_len, 2);
}

#[test]
fn test_flush_decoded_buf_full_copy() {
    struct Decoder {
        decoded_len: usize,
        decoded_offset: usize,
        decoded_chunk_buffer: Vec<u8>,
    }

    let mut decoder = Decoder {
        decoded_len: 5,
        decoded_offset: 0,
        decoded_chunk_buffer: vec![1, 2, 3, 4, 5],
    };

    let mut buf: [u8; 5] = [0; 5];

    let result = decoder.flush_decoded_buf(&mut buf).unwrap();
    assert_eq!(result, 5);
    assert_eq!(buf, [1, 2, 3, 4, 5]);
    assert_eq!(decoder.decoded_offset, 5);
    assert_eq!(decoder.decoded_len, 0);
}

