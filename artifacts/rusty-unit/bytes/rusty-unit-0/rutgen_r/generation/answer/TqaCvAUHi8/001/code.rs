// Answer 0

#[test]
fn test_chunk_with_non_empty_bytes() {
    let data = vec![1, 2, 3, 4, 5];
    let byte_slice = bytes::BytesMut::from(data);
    assert_eq!(byte_slice.chunk(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_with_empty_bytes() {
    let data: Vec<u8> = Vec::new();
    let byte_slice = bytes::BytesMut::from(data);
    assert_eq!(byte_slice.chunk(), &[]);
}

#[test]
fn test_chunk_with_large_bytes() {
    let data = (0..1000).map(|x| x as u8).collect::<Vec<u8>>();
    let byte_slice = bytes::BytesMut::from(data);
    assert_eq!(byte_slice.chunk(), &(0..1000).map(|x| x as u8).collect::<Vec<u8>>()[..]);
}

