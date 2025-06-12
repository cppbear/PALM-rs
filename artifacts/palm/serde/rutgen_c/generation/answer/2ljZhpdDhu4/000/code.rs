// Answer 0

#[test]
fn test_write_str_success() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 0,
    };
    
    let result = buf.write_str("hello");
    assert!(result.is_ok());
    assert_eq!(&buffer[..5], b"hello");
    assert_eq!(buf.offset, 5);
}

#[test]
fn test_write_str_partial_fit() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 5,
    };
    
    let result = buf.write_str("world");
    assert!(result.is_ok());
    assert_eq!(&buffer[5..10], b"world");
    assert_eq!(buf.offset, 10);
}

#[test]
fn test_write_str_overflow() {
    let mut buffer = [0u8; 10];
    let mut buf = Buf {
        bytes: &mut buffer,
        offset: 8,
    };
    
    let result = buf.write_str("test");
    assert!(result.is_err());
    assert_eq!(buf.offset, 8);
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
    assert_eq!(buf.offset, 0);
    assert_eq!(&buffer[..], &[0u8; 10]);
}

