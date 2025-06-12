// Answer 0

#[test]
fn test_promote_to_shared_ref_count_one() {
    let mut bytes_mut = BytesMut::with_capacity(16);
    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
fn test_promote_to_shared_ref_count_two() {
    let mut bytes_mut = BytesMut::with_capacity(32);
    unsafe {
        bytes_mut.promote_to_shared(2);
    }
}

#[test]
#[should_panic]
fn test_promote_to_shared_invalid_ref_count_zero() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.promote_to_shared(0);
    }
}

#[test]
#[should_panic]
fn test_promote_to_shared_invalid_ref_count_greater_than_two() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.promote_to_shared(3);
    }
}

