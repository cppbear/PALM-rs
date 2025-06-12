// Answer 0

#[test]
fn test_advance_mut_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        // Initially, len is 0, so remaining is 10.
        bytes_mut.set_len(0);
        bytes_mut.advance_mut(10); // This should succeed, advancing to len 10.
        assert_eq!(bytes_mut.len(), 10);
    }
}

#[test]
#[should_panic(expected = "advance out of bounds")]
fn test_advance_mut_exceeding_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(0);
        bytes_mut.advance_mut(11); // This should panic as cnt > remaining (10).
    }
}

#[test]
fn test_advance_mut_with_no_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    unsafe {
        bytes_mut.set_len(5); // Setting len to capacity, remaining is 0.
        bytes_mut.advance_mut(0); // This should succeed, len remains 5.
        assert_eq!(bytes_mut.len(), 5);
    }
}

#[test]
fn test_advance_mut_zero_bytes() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    unsafe {
        bytes_mut.set_len(0); // Set len to 0, remaining is 5.
        bytes_mut.advance_mut(0); // This should always succeed.
        assert_eq!(bytes_mut.len(), 0); // Len should remain 0.
    }
}

#[test]
fn test_advance_mut_with_exact_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(15);
    unsafe {
        bytes_mut.set_len(5); // Setting len to 5, remaining is 10.
        bytes_mut.advance_mut(10); // This should succeed, advancing to len 15.
        assert_eq!(bytes_mut.len(), 15);
    }
}

