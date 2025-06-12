// Answer 0

#[test]
fn test_reserve_inner_case_1() {
    let mut bytes_mut = BytesMut::with_capacity(3);
    unsafe {
        bytes_mut.set_len(3);
    }
    bytes_mut.reserve_inner(0, true);
}

#[test]
fn test_reserve_inner_case_2() {
    let mut bytes_mut = BytesMut::with_capacity(6);
    unsafe {
        bytes_mut.set_len(3);
        bytes_mut.ptr = vptr(bytes_mut.ptr.as_ptr().add(3)); // simulate the off situation
        bytes_mut.cap = 6; // simulate capacity
    }
    bytes_mut.reserve_inner(0, true);
}

