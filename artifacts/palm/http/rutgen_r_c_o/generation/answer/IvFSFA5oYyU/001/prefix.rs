// Answer 0

#[test]
fn test_hash_with_lower_true_buf_within_capacity() {
    let buf = BytesMut::from(&b"test"[..]);
    let lower = true;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let maybe_lower = MaybeLower { buf: &buf, lower };
    maybe_lower.hash(&mut hasher);
}

#[test]
fn test_hash_with_lower_true_buf_at_min_capacity() {
    let buf = BytesMut::from(&b"a"[..]);
    let lower = true;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let maybe_lower = MaybeLower { buf: &buf, lower };
    maybe_lower.hash(&mut hasher);
}

#[test]
fn test_hash_with_lower_true_buf_at_max_capacity() {
    let buf = BytesMut::from(&b"abcdefghijklmnopqrstuvwxyzabcdefg"[..]);
    let lower = true;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let maybe_lower = MaybeLower { buf: &buf, lower };
    maybe_lower.hash(&mut hasher);
}

#[test]
fn test_hash_with_lower_true_buf_full_capacity() {
    let buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(&b"whatever we want to test this buffer capacity"[..]); // using a long input
    let lower = true;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let maybe_lower = MaybeLower { buf: &buf, lower };
    maybe_lower.hash(&mut hasher);
}

#[test]
fn test_hash_with_lower_true_empty_buf() {
    let buf = BytesMut::with_capacity(64);
    let lower = true;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let maybe_lower = MaybeLower { buf: &buf, lower };
    maybe_lower.hash(&mut hasher);
}

