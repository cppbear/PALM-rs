// Answer 0

#[test]
fn test_put_bytes_with_exact_capacity() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 6);
    }
}

#[test]
fn test_put_bytes_with_zero_count() {
    let mut dst = [0; 6];
    {
        let mut buf = &mut dst[..];
        buf.put_bytes(b'a', 0);
    }
}

