// Answer 0

#[test]
fn test_str_read_discard_valid() {
    let mut data: &[u8] = b"Hello, World!";
    let slice_read = SliceRead { slice: data, index: 0 };
    let mut str_read = StrRead { delegate: slice_read };
    
    str_read.discard();
}

#[test]
fn test_str_read_discard_valid_at_end() {
    let data: &[u8] = b"Goodbye!";
    let mut slice_read = SliceRead { slice: data, index: data.len() };
    let mut str_read = StrRead { delegate: slice_read };
    
    str_read.discard();
}

#[test]
fn test_str_read_discard_mid_slice() {
    let data: &[u8] = b"Testing discard!";
    let mut slice_read = SliceRead { slice: data, index: 5 };
    let mut str_read = StrRead { delegate: slice_read };
    
    str_read.discard();
}

