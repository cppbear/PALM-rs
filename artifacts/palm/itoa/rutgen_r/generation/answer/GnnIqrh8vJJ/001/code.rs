// Answer 0

#[test]
fn test_new_buffer() {
    use std::mem::MaybeUninit;

    struct Buffer {
        bytes: [MaybeUninit<u8>; i128::MAX_STR_LEN],
    }

    let buffer = new();
    let expected_bytes = [MaybeUninit::<u8>::uninit(); i128::MAX_STR_LEN];

    for (byte, expected_byte) in buffer.bytes.iter().zip(expected_bytes.iter()) {
        assert_eq!(byte, expected_byte);
    }
}

