// Answer 0

#[test]
fn test_copy_to_bytes_a_rem_equal_len() {
    let mut buf_a = BytesMut::with_capacity(10);
    buf_a.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut buf_b = BytesMut::with_capacity(5);
    let chain_buf = Chain { a: buf_a, b: buf_b };
    let len = chain_buf.a.remaining();
    chain_buf.copy_to_bytes(len);
}

#[test]
fn test_copy_to_bytes_a_rem_greater_than_len() {
    let mut buf_a = BytesMut::with_capacity(10);
    buf_a.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut buf_b = BytesMut::with_capacity(5);
    let chain_buf = Chain { a: buf_a, b: buf_b };
    let len = 5; // a_rem > len
    chain_buf.copy_to_bytes(len);
}

#[test]
fn test_copy_to_bytes_a_rem_zero() {
    let mut buf_a = BytesMut::zeroed(0); // Zero remaining bytes
    let mut buf_b = BytesMut::with_capacity(5);
    buf_b.extend_from_slice(&[10, 20, 30, 40, 50]);
    let chain_buf = Chain { a: buf_a, b: buf_b };
    let len = 5; // Should copy entirely from b
    chain_buf.copy_to_bytes(len);
}

#[test]
fn test_copy_to_bytes_a_rem_non_zero() {
    let mut buf_a = BytesMut::with_capacity(10);
    buf_a.extend_from_slice(&[1, 2, 3, 4, 5]);
    let mut buf_b = BytesMut::with_capacity(10);
    buf_b.extend_from_slice(&[6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    let chain_buf = Chain { a: buf_a, b: buf_b };
    let len = 7; // a_rem < len
    chain_buf.copy_to_bytes(len);
}

