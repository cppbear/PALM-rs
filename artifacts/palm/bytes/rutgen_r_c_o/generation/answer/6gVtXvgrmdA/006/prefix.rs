// Answer 0

#[test]
fn test_promote_to_shared_ref_count_1() {
    let mut bytes_mut = BytesMut::with_capacity(16);
    unsafe {
        bytes_mut.data = (KIND_VEC as *mut usize) << VEC_POS_OFFSET; // Set kind to KIND_VEC
        bytes_mut.ptr = NonNull::new(&mut bytes_mut); // Assure it points to a valid memory
        bytes_mut.len = 5; // Set length
        bytes_mut.cap = 16; // Set capacity
        bytes_mut.promote_to_shared(1);
    }
}

#[test]
fn test_promote_to_shared_ref_count_2() {
    let mut bytes_mut = BytesMut::with_capacity(32);
    unsafe {
        bytes_mut.data = (KIND_VEC as *mut usize) << VEC_POS_OFFSET; // Set kind to KIND_VEC
        bytes_mut.ptr = NonNull::new(&mut bytes_mut); // Assure it points to a valid memory
        bytes_mut.len = 10; // Set length
        bytes_mut.cap = 32; // Set capacity
        bytes_mut.promote_to_shared(2);
    }
}

#[test]
#[should_panic]
fn test_promote_to_shared_invalid_ref_count() {
    let mut bytes_mut = BytesMut::new();
    unsafe {
        bytes_mut.data = (KIND_VEC as *mut usize) << VEC_POS_OFFSET; // Set kind to KIND_VEC
        bytes_mut.ptr = NonNull::new(&mut bytes_mut); // Assure it points to a valid memory
        bytes_mut.len = 5; // Set length
        bytes_mut.cap = 8; // Set capacity
        bytes_mut.promote_to_shared(3); // Should panic
    }
}

#[test]
#[should_panic]
fn test_promote_to_shared_invalid_kind() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    unsafe {
        bytes_mut.data = (KIND_ARC as *mut usize) << VEC_POS_OFFSET; // Set kind to KIND_ARC
        bytes_mut.ptr = NonNull::new(&mut bytes_mut); // Assure it points to a valid memory
        bytes_mut.len = 15; // Set length
        bytes_mut.cap = 64; // Set capacity
        bytes_mut.promote_to_shared(1); // Should panic
    }
}

