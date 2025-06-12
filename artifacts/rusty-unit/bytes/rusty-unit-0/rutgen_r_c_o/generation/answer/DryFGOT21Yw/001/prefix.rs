// Answer 0

#[test]
fn test_get_vec_pos_with_zero() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    unsafe { bytes_mut.set_len(1) };
    let data_addr = (&mut bytes_mut as *mut _ as usize) | KIND_VEC; // make it KIND_VEC
    bytes_mut.data = data_addr as *mut Shared;
    unsafe {
        let result = bytes_mut.get_vec_pos();
    }
}

#[test]
fn test_get_vec_pos_with_max() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    unsafe { bytes_mut.set_len(1) };
    let data_addr = ((usize::MAX >> VEC_POS_OFFSET) << VEC_POS_OFFSET) | KIND_VEC; // ensure it's within bounds
    bytes_mut.data = data_addr as *mut Shared;
    unsafe {
        let result = bytes_mut.get_vec_pos();
    }
}

#[test]
fn test_get_vec_pos_with_mid_value() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    unsafe { bytes_mut.set_len(1) };
    let data_addr = ((usize::MAX >> VEC_POS_OFFSET) >> 1 << VEC_POS_OFFSET) | KIND_VEC; // mid-value for tests
    bytes_mut.data = data_addr as *mut Shared;
    unsafe {
        let result = bytes_mut.get_vec_pos();
    }
}

#[test]
#[should_panic]
fn test_get_vec_pos_with_non_vec_kind() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    unsafe { bytes_mut.set_len(1) };
    let data_addr = 1; // make it a non Vec kind
    bytes_mut.data = data_addr as *mut Shared;
    unsafe {
        let _ = bytes_mut.get_vec_pos(); // should trigger panic
    }
}

