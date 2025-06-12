// Answer 0

#[test]
fn test_write_str_exact_fit() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 0,
    };
    let result = buf.write_str("abcde");
    assert!(result.is_ok());
    assert_eq!(&buffer[..5], b"abcde");
    assert_eq!(buf.offset, 5);
}

#[test]
fn test_write_str_empty() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 0,
    };
    let result = buf.write_str("");
    assert!(result.is_ok());
    assert_eq!(&buffer[..0], b"");
    assert_eq!(buf.offset, 0);
}

#[test]
fn test_write_str_fill_buffer() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 5,
    };
    let result = buf.write_str("fghij");
    assert!(result.is_ok());
    assert_eq!(&buffer[..10], b"abcdefghij");
    assert_eq!(buf.offset, 10);
}

#[test]
fn test_write_str_panic_condition() {
    let mut buffer = [0u8; 3];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 0,
    };
    let result = buf.write_str("abcd");
    assert!(result.is_err());
}

