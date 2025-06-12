// Answer 0

#[test]
fn test_advance_unchecked_small_increment() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.advance_unchecked(1);
    }
}

#[test]
fn test_advance_unchecked_mid_increment() {
    let mut buffer = BytesMut::with_capacity(20);
    unsafe {
        buffer.advance_unchecked(15);
    }
}

#[test]
fn test_advance_unchecked_large_increment() {
    let mut buffer = BytesMut::with_capacity(100);
    unsafe {
        buffer.advance_unchecked(90);
    }
}

#[test]
fn test_advance_unchecked_at_max_pos() {
    let mut buffer = BytesMut::with_capacity(MAX_VEC_POS);
    unsafe {
        buffer.advance_unchecked(MAX_VEC_POS);
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_exceeding_capacity() {
    let mut buffer = BytesMut::with_capacity(10);
    unsafe {
        buffer.advance_unchecked(11);
    }
}

