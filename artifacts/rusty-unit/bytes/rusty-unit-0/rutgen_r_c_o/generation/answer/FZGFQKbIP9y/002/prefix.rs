// Answer 0

#[test]
#[should_panic]
fn test_set_len_exceeds_capacity() {
    let mut b = BytesMut::with_capacity(10);
    unsafe {
        b.set_len(11); // len exceeds cap
    }
}

#[test]
#[should_panic]
fn test_set_len_max_value() {
    let mut b = BytesMut::with_capacity(20);
    unsafe {
        b.set_len(usize::MAX); // len exceeds cap
    }
}

#[test]
#[should_panic]
fn test_set_len_exceeds_after_initialization() {
    let mut b = BytesMut::new();
    unsafe {
        b.set_len(1); // len exceeds cap (capacity is 0)
    }
}


