// Answer 0

#[test]
fn test_borrow_empty() {
    let bytes = Bytes::new();
    bytes.borrow();
}

#[test]
fn test_borrow_single_byte() {
    let bytes = Bytes::from_static(b"a");
    bytes.borrow();
}

#[test]
fn test_borrow_two_bytes() {
    let bytes = Bytes::from_static(b"ab");
    bytes.borrow();
}

#[test]
fn test_borrow_edge_case_max_len() {
    let large_bytes = vec![0u8; std::usize::MAX];
    let bytes = Bytes::from_owner(large_bytes);
    bytes.borrow();
}

