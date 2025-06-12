// Answer 0

#[test]
fn test_put_with_empty_buffer() {
    let mut bytes_mut = BytesMut::new();
    let empty_buf: &[u8] = &[];
    bytes_mut.put(empty_buf);
    assert_eq!(bytes_mut.len(), 0);
}

#[test]
fn test_put_with_data() {
    let mut bytes_mut = BytesMut::new();
    let data_buf: &[u8] = &[1, 2, 3, 4, 5];
    bytes_mut.put(data_buf);
    assert_eq!(bytes_mut.len(), data_buf.len());
    assert_eq!(bytes_mut.as_slice(), data_buf);
}

#[test]
fn test_put_with_large_buffer() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    let large_buf: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    bytes_mut.put(large_buf);
    assert_eq!(bytes_mut.len(), large_buf.len());
    assert_eq!(bytes_mut.as_slice(), large_buf);
}

#[test]
fn test_put_with_non_contiguous_data() {
    let mut bytes_mut = BytesMut::new();
    let data1: &[u8] = &[1, 2, 3];
    let data2: &[u8] = &[4, 5, 6, 7];
    bytes_mut.put(data1);
    bytes_mut.put(data2);
    assert_eq!(bytes_mut.len(), data1.len() + data2.len());
    assert_eq!(bytes_mut.as_slice(), &[1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_put_reserves_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    let data_buf: &[u8] = &[1, 2, 3, 4, 5, 6];
    bytes_mut.put(data_buf);
    assert!(bytes_mut.capacity() >= data_buf.len());
    assert_eq!(bytes_mut.len(), data_buf.len());
}

