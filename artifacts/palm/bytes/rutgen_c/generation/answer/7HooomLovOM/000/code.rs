// Answer 0

#[test]
fn test_put_bytes_empty_vector() {
    let mut buf = Vec::new();
    buf.put_bytes(0xAB, 5);
    assert_eq!(buf.len(), 5);
    assert_eq!(&buf[..], &[0xAB, 0xAB, 0xAB, 0xAB, 0xAB]);
}

#[test]
fn test_put_bytes_non_empty_vector() {
    let mut buf = Vec::from([0x01, 0x02, 0x03]);
    buf.put_bytes(0xAB, 3);
    assert_eq!(buf.len(), 6);
    assert_eq!(&buf[..], &[0x01, 0x02, 0x03, 0xAB, 0xAB, 0xAB]);
}

#[test]
fn test_put_bytes_large_count() {
    let mut buf = Vec::new();
    buf.put_bytes(0xAA, usize::MAX - 1);
    assert_eq!(buf.len(), usize::MAX - 1);
}

#[should_panic]
fn test_put_bytes_overflow() {
    let mut buf = Vec::new();
    buf.put_bytes(0xAA, usize::MAX);
}

#[test]
fn test_put_bytes_zero_count() {
    let mut buf = Vec::from([0x01, 0x02]);
    buf.put_bytes(0xAB, 0);
    assert_eq!(buf.len(), 2);
    assert_eq!(&buf[..], &[0x01, 0x02]);
}

