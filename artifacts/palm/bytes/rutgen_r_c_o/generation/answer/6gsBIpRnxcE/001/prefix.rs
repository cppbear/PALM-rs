// Answer 0

#[test]
fn test_inc_start_at_zero() {
    let mut bytes = Bytes::new();
    let len = bytes.len();
    unsafe { bytes.inc_start(0) };
}

#[test]
fn test_inc_start_at_len() {
    let mut bytes = Bytes::from_static(b"test");
    let len = bytes.len();
    unsafe { bytes.inc_start(len) };
}

#[should_panic]
fn test_inc_start_above_len() {
    let mut bytes = Bytes::from_static(b"test");
    let len = bytes.len();
    unsafe { bytes.inc_start(len + 1) };
}

