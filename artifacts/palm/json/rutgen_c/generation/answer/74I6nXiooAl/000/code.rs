// Answer 0

#[test]
fn test_str_read_new() {
    let input_str = "Hello, world!";
    let str_read = StrRead::new(input_str);
    
    assert_eq!(str_read.delegate.slice, input_str.as_bytes());
    assert_eq!(str_read.delegate.index, 0);
}

#[test]
fn test_slice_read_new() {
    let input_slice: &[u8] = b"Test slice";
    let slice_read = SliceRead::new(input_slice);
    
    assert_eq!(slice_read.slice, input_slice);
    assert_eq!(slice_read.index, 0);
}

