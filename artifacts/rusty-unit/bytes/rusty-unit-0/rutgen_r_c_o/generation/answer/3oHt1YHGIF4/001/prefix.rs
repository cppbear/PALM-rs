// Answer 0

#[test]
fn test_into_iter_non_empty() {
    let buf: &[u8] = b"abc";
    let iter = IntoIter::new(buf);
}

#[test]
fn test_into_iter_empty() {
    let buf: &[u8] = b"";
    let iter = IntoIter::new(buf);
}

#[test]
fn test_into_iter_max_size() {
    let buf: &[u8] = &[0; 256];
    let iter = IntoIter::new(buf);
}

