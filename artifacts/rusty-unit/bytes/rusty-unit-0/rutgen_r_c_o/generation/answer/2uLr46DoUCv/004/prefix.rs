// Answer 0

#[test]
fn test_truncate_equal_length() {
    let mut buf = Bytes::from_static(&b"hello world"[..]);
    buf.truncate(11);  // len == self.len
}

#[test]
fn test_truncate_zero_length() {
    let mut buf = Bytes::from_static(&b"hello world"[..]);
    buf.truncate(0);  // len == 0
}

#[test]
fn test_truncate_non_empty() {
    let mut buf = Bytes::from_static(&b"hello"[..]);
    buf.truncate(5);  // len == self.len
}

#[test]
fn test_truncate_exceeding_length() {
    let mut buf = Bytes::from_static(&b"hello"[..]);
    buf.truncate(6);  // len > self.len, should have no effect
}

#[test]
fn test_truncate_empty() {
    let mut buf = Bytes::new();  // self.len == 0
    buf.truncate(0);  // len == 0
}

