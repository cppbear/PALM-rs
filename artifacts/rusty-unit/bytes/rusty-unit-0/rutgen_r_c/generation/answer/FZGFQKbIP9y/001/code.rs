// Answer 0

#[test]
fn test_set_len_equal_to_capacity() {
    let mut b = BytesMut::with_capacity(10);
    unsafe {
        b.set_len(10);
    }
    assert_eq!(b.len(), 10);
}

#[test]
fn test_set_len_smaller_than_capacity() {
    let mut b = BytesMut::with_capacity(10);
    unsafe {
        b.set_len(5);
    }
    assert_eq!(b.len(), 5);
}

#[test]
#[should_panic(expected = "set_len out of bounds")]
fn test_set_len_exceed_capacity() {
    let mut b = BytesMut::with_capacity(10);
    unsafe {
        b.set_len(11);
    }
}

#[test]
fn test_set_len_zero() {
    let mut b = BytesMut::with_capacity(10);
    unsafe {
        b.set_len(0);
    }
    assert_eq!(b.len(), 0);
}

