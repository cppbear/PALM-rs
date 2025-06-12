// Answer 0

#[derive(Debug)]
struct Buffer {
    bytes: [std::mem::MaybeUninit<u8>; 39], // since i128::MAX_STR_LEN = 39
}

#[test]
fn test_buffer_initialization() {
    let buffer = new();
    assert_eq!(buffer.bytes.len(), 39);
    for byte in buffer.bytes.iter() {
        assert!(byte.is_uninit());
    }
}

#[test]
fn test_buffer_new_function() {
    let buffer = new();
    assert_eq!(buffer.bytes[0].as_ptr() as usize != 0, true);
}

