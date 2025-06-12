// Answer 0

#[test]
fn test_slice_valid_range() {
    let bytes = Bytes::from_static(b"hello world");

    let sliced = bytes.slice(2..5);
    
    assert_eq!(&sliced[..], b"llo");
}

#[test]
#[should_panic(expected = "range start must not be greater than end")]
fn test_slice_invalid_range_start_greater_than_end() {
    let bytes = Bytes::from_static(b"hello world");
    
    let _ = bytes.slice(5..2);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_invalid_end_out_of_bounds() {
    let bytes = Bytes::from_static(b"hello world");
    
    let _ = bytes.slice(0..15);
}

#[test]
fn test_slice_empty_range() {
    let bytes = Bytes::from_static(b"hello world");
    
    let sliced = bytes.slice(5..5);
    
    assert_eq!(sliced.len(), 0);
}

#[test]
#[should_panic(expected = "range end out of bounds")]
fn test_slice_exceeding_end_boundary() {
    let bytes = Bytes::from_static(b"hello world");
    
    let _ = bytes.slice(10..15);
}

