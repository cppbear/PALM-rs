// Answer 0

#[test]
fn test_advance_mut_within_bounds() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_mut(5);
    }
    assert_eq!(bytes_mut.len(), 5);
}

#[test]
#[should_panic]
fn test_advance_mut_exceeding_bounds() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_mut(11);
    }
}

#[test]
fn test_advance_mut_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_mut(10);
    }
    assert_eq!(bytes_mut.len(), 10);
}

#[test]
fn test_advance_mut_zero_increment() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_mut(0);
    }
    assert_eq!(bytes_mut.len(), 0);
}

#[test]
fn test_advance_mut_multiple_calls() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe {
        bytes_mut.advance_mut(5);
        bytes_mut.advance_mut(10);
    }
    assert_eq!(bytes_mut.len(), 15);
}

