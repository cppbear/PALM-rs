// Answer 0

#[test]
fn test_advance_unchecked_count_zero() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.advance_unchecked(0);
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_count_exceeds_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    unsafe {
        bytes_mut.advance_unchecked(6);
    }
}

#[test]
#[should_panic]
fn test_advance_unchecked_count_exceeds_max_vec_pos() {
    let mut bytes_mut = BytesMut::with_capacity(MAX_VEC_POS);
    unsafe {
        bytes_mut.advance_unchecked(MAX_VEC_POS + 1);
    }
}

