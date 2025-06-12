// Answer 0

#[test]
fn test_hash_non_empty_data() {
    let data = Bytes::from_static(b"example");
    let authority = Authority { data: ByteStr { bytes: data } };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    authority.hash(&mut hasher);
}

#[test]
fn test_hash_large_data() {
    let data = Bytes::from_static(b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabc");
    let authority = Authority { data: ByteStr { bytes: data } };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    authority.hash(&mut hasher);
}

#[test]
fn test_hash_empty_data() {
    let data = Bytes::from_static(b"");
    let authority = Authority { data: ByteStr { bytes: data } };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    authority.hash(&mut hasher);
}

