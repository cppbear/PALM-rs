// Answer 0

#[test]
fn test_write_fmt_empty_string() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };
    let result = bytes_mut.write_fmt(format_args!(""));
    assert!(result.is_ok());
}

#[test]
fn test_write_fmt_simple_string() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 10,
        data: std::ptr::null_mut(),
    };
    let result = bytes_mut.write_fmt(format_args!("Hello, {}!", "World"));
    assert!(result.is_ok());
}

#[test]
fn test_write_fmt_large_string() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 100,
        data: std::ptr::null_mut(),
    };
    let input_string = "a".repeat(80); // Create a large input string
    let result = bytes_mut.write_fmt(format_args!("{}", input_string));
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_write_fmt_exceeding_capacity() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(Box::into_raw(Box::new(0u8))).unwrap(),
        len: 0,
        cap: 5, // Set small capacity
        data: std::ptr::null_mut(),
    };
    let _ = bytes_mut.write_fmt(format_args!("This string is too long for capacity"));
}

