// Answer 0

#[test]
fn test_clear_on_empty_buffer() {
    let mut buf = Bytes::new();
    buf.clear();
}

#[test]
fn test_clear_on_non_empty_buffer() {
    let data: &[u8] = b"test data";
    let mut buf = Bytes::from_static(data);
    buf.clear();
}

#[test]
fn test_clear_after_split() {
    let data: &[u8] = b"clear test";
    let mut buf = Bytes::from_static(data);
    let _split_buf = buf.split_off(5);
    buf.clear();
}

#[test]
fn test_clear_after_truncate() {
    let data: &[u8] = b"data before clear";
    let mut buf = Bytes::from_static(data);
    buf.truncate(5);
    buf.clear();
}

#[test]
fn test_clear_on_large_buffer() {
    let data: Vec<u8> = (0..usize::MAX).map(|x| x as u8).collect();
    let mut buf = Bytes::from_owner(data);
    buf.clear();
}

