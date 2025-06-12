// Answer 0

#[test]
fn test_split_to_equal_length() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(5);
    
    assert_eq!(bytes.len(), 0);
    assert_eq!(result.len(), 5);
    assert_eq!(unsafe { slice::from_raw_parts(result.ptr, result.len) }, b"hello");
}

#[test]
fn test_split_to_zero_length() {
    let mut bytes = Bytes::from_static(b"hello");
    let result = bytes.split_to(0);
    
    assert_eq!(bytes.len(), 5);
    assert_eq!(result.len(), 0);
}

#[test]
#[should_panic(expected = "split_to out of bounds")]
fn test_split_to_out_of_bounds() {
    let mut bytes = Bytes::from_static(b"hello");
    bytes.split_to(6);
}

#[test]
fn test_split_to_non_empty() {
    let mut bytes = Bytes::from_static(b"hello world");
    let result = bytes.split_to(5);
    
    assert_eq!(bytes.len(), 6);
    assert_eq!(result.len(), 5);
    assert_eq!(unsafe { slice::from_raw_parts(result.ptr, result.len) }, b"hello");
    assert_eq!(unsafe { slice::from_raw_parts(bytes.ptr, bytes.len) }, b" world");
}

