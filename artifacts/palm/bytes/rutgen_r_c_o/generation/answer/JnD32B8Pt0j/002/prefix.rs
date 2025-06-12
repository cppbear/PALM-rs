// Answer 0

#[test]
fn test_set_vec_pos_zero() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.set_vec_pos(0);
    }
}

#[test]
#[should_panic]
fn test_set_vec_pos_exceeding_max_vec_pos() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.set_vec_pos(MAX_VEC_POS + 1);
    }
}

