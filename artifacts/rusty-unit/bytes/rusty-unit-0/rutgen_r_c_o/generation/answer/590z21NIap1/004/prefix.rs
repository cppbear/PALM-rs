// Answer 0

#[test]
fn test_advance_unchecked_case_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_unchecked(0);
    }
}

#[test]
fn test_advance_unchecked_case_within_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0);
    unsafe {
        bytes_mut.advance_unchecked(5);
    }
}

#[test]
fn test_advance_unchecked_case_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0);
    unsafe {
        bytes_mut.advance_unchecked(10);
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_case_exceeding_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(10, 0);
    unsafe {
        bytes_mut.advance_unchecked(11);
    }
}

#[test]
fn test_advance_unchecked_case_non_vec_kind() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.resize(5, 0);
    unsafe {
        bytes_mut.advance_unchecked(3);
    }
}

