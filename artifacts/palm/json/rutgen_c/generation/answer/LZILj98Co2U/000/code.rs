// Answer 0

#[test]
fn test_slice_read_new() {
    let input: &[u8] = b"example input";
    let slice_read = SliceRead::new(input);
    
    assert_eq!(slice_read.slice, input);
    assert_eq!(slice_read.index, 0);
    #[cfg(feature = "raw_value")]
    assert_eq!(slice_read.raw_buffering_start_index, 0);
}

#[test]
fn test_slice_read_index_initial() {
    let input: &[u8] = b"another test";
    let slice_read = SliceRead::new(input);
    
    assert_eq!(slice_read.index, 0);
}

#[test]
fn test_slice_read_empty_slice() {
    let input: &[u8] = b"";
    let slice_read = SliceRead::new(input);
    
    assert_eq!(slice_read.slice, input);
    assert_eq!(slice_read.index, 0);
}

