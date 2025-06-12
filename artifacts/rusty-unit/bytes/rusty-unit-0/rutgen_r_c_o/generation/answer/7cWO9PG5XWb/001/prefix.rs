// Answer 0

#[test]
fn test_reserve_when_additional_equals_remaining() {
    let mut buf = BytesMut::with_capacity(128);
    buf.resize(64, 0);
    buf.reserve(64);
}

#[test]
fn test_reserve_when_additional_is_zero() {
    let mut buf = BytesMut::with_capacity(128);
    buf.resize(64, 0);
    buf.reserve(0);
}

#[test]
fn test_reserve_for_full_capacity() {
    let mut buf = BytesMut::with_capacity(128);
    buf.resize(127, 0);
    buf.reserve(1);
}

#[test]
fn test_reserve_when_empty() {
    let mut buf = BytesMut::with_capacity(128);
    buf.reserve(128);
}

#[test]
fn test_reserve_when_max_capacity() {
    let mut buf = BytesMut::with_capacity(usize::MAX);
    buf.reserve(0);
}

