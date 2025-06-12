// Answer 0

#[test]
fn test_put_with_non_empty_src() {
    let mut dest = BytesMut::new();
    let src_data = vec![1u8, 2, 3, 4, 5];
    let mut src = BytesMut::from_vec(src_data);
    dest.put(src);
}

#[test]
fn test_put_with_empty_src() {
    let mut dest = BytesMut::new();
    let src_data: Vec<u8> = vec![];
    let mut src = BytesMut::from_vec(src_data);
    dest.put(src);
}

#[test]
fn test_put_with_single_element_src() {
    let mut dest = BytesMut::new();
    let src_data = vec![10u8];
    let mut src = BytesMut::from_vec(src_data);
    dest.put(src);
}

#[test]
fn test_put_with_large_src() {
    let mut dest = BytesMut::new();
    let src_data = vec![0u8; 1024]; // Large buffer of 1024 zeroes
    let mut src = BytesMut::from_vec(src_data);
    dest.put(src);
}

#[test]
fn test_put_with_multiple_chunks() {
    let mut dest = BytesMut::new();
    let src_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut src = BytesMut::from_vec(src_data);
    dest.put(src);
}

