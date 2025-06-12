// Answer 0

#[test]
fn test_promote_to_shared_valid_case_1() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.ptr = NonNull::new(Box::into_raw(Box::new([0u8; 10][..])) as *mut u8).unwrap();
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    bytes_mut.data = (2 << ORIGINAL_CAPACITY_OFFSET) | (0 << VEC_POS_OFFSET);
    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
fn test_promote_to_shared_valid_case_2() {
    let mut bytes_mut = BytesMut::with_capacity(15);
    bytes_mut.ptr = NonNull::new(Box::into_raw(Box::new([1u8; 15][..])) as *mut u8).unwrap();
    bytes_mut.len = 15;
    bytes_mut.cap = 15;
    bytes_mut.data = (3 << ORIGINAL_CAPACITY_OFFSET) | (1 << VEC_POS_OFFSET);
    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
#[should_panic]
fn test_promote_to_shared_invalid_ref_cnt_1() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.ptr = NonNull::new(Box::into_raw(Box::new([0u8; 10][..])) as *mut u8).unwrap();
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    bytes_mut.data = (2 << ORIGINAL_CAPACITY_OFFSET) | (2 << VEC_POS_OFFSET);
    unsafe {
        bytes_mut.promote_to_shared(2);
    }
}

#[test]
fn test_promote_to_shared_zero_offset() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.ptr = NonNull::new(Box::into_raw(Box::new([0u8; 10][..])) as *mut u8).unwrap();
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    bytes_mut.data = (2 << ORIGINAL_CAPACITY_OFFSET) | (0 << VEC_POS_OFFSET);
    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
fn test_promote_to_shared_edge_case_high_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(17);
    bytes_mut.ptr = NonNull::new(Box::into_raw(Box::new([1u8; 17][..])) as *mut u8).unwrap();
    bytes_mut.len = 17;
    bytes_mut.cap = 17;
    bytes_mut.data = (3 << ORIGINAL_CAPACITY_OFFSET) | (0 << VEC_POS_OFFSET);
    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
fn test_promote_to_shared_edge_case_low_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.ptr = NonNull::new(Box::into_raw(Box::new([7u8; 10][..])) as *mut u8).unwrap();
    bytes_mut.len = 10;
    bytes_mut.cap = 10;
    bytes_mut.data = (2 << ORIGINAL_CAPACITY_OFFSET) | (3 << VEC_POS_OFFSET);
    unsafe {
        bytes_mut.promote_to_shared(1);
    }
}

