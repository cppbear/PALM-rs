// Answer 0

#[test]
fn test_extend_from_slice_zero_capacity() {
    let mut buf = BytesMut::with_capacity(0);
    buf.extend_from_slice(b"a");
}

#[test]
fn test_extend_from_slice_non_zero_capacity() {
    let mut buf = BytesMut::with_capacity(10);
    buf.extend_from_slice(b"abc");
}

#[test]
fn test_extend_from_slice_exact_fit() {
    let mut buf = BytesMut::with_capacity(3);
    buf.extend_from_slice(b"abc");
}

#[test]
fn test_extend_from_slice_over_capacity() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b"abcde");
}

#[test]
fn test_extend_from_slice_multiple_calls() {
    let mut buf = BytesMut::with_capacity(5);
    buf.extend_from_slice(b"abc");
    buf.extend_from_slice(b"de");
}

#[test]
fn test_extend_from_slice_edge_case_large_input() {
    let mut buf = BytesMut::with_capacity(100);
    let large_input = vec![1u8; 100];
    buf.extend_from_slice(&large_input);
}

