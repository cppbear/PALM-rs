// Answer 0

#[test]
fn test_remaining_mut_zero_length() {
    let bytes_mut = BytesMut::new();
    assert_eq!(bytes_mut.remaining_mut(), usize::MAX);
}

#[test]
fn test_remaining_mut_non_zero_length() {
    let mut bytes_mut = BytesMut::with_capacity(100);
    unsafe { bytes_mut.set_len(10) }; // setting length to 10
    assert_eq!(bytes_mut.remaining_mut(), usize::MAX - 10);
}

#[test]
fn test_remaining_mut_at_max_length() {
    let mut bytes_mut = BytesMut::with_capacity(100);
    unsafe { bytes_mut.set_len(usize::MAX) };
    assert_eq!(bytes_mut.remaining_mut(), 0);
}

#[test]
#[should_panic]
fn test_remaining_mut_overflow() {
    let mut bytes_mut = BytesMut::with_capacity(100);
    unsafe { bytes_mut.set_len(usize::MAX - 1) }; // setting length to (MAX - 1)
    let _ = bytes_mut.remaining_mut(); // this may cause panic on overflow
}

