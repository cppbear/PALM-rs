// Answer 0

#[test]
fn test_reserve_inner_with_non_allocated_buffer_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(16);
    unsafe {
        bytes_mut.set_len(8);
        bytes_mut.ptr = vptr(bytes_mut.ptr.as_ptr());
        bytes_mut.data = invalid_ptr((0 << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC);
    }
    let additional = 10; // additional is greater than capacity - len + off
    let allocate = false;

    let result = bytes_mut.reserve_inner(additional, allocate);
}

#[test]
fn test_reserve_inner_with_full_capacity_and_no_allocation() {
    let mut bytes_mut = BytesMut::with_capacity(8);
    unsafe {
        bytes_mut.set_len(8);
        bytes_mut.ptr = vptr(bytes_mut.ptr.as_ptr());
        bytes_mut.data = invalid_ptr((0 << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC);
    }
    let additional = 1; // no space available
    let allocate = false;

    let result = bytes_mut.reserve_inner(additional, allocate);
}

#[test]
fn test_reserve_inner_with_zero_additional() {
    let mut bytes_mut = BytesMut::with_capacity(32);
    unsafe {
        bytes_mut.set_len(20);
        bytes_mut.ptr = vptr(bytes_mut.ptr.as_ptr());
        bytes_mut.data = invalid_ptr((0 << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC);
    }
    let additional = 0; // no additional space requested
    let allocate = false;

    let result = bytes_mut.reserve_inner(additional, allocate);
}

#[test]
fn test_reserve_inner_with_excessive_additional_space() {
    let mut bytes_mut = BytesMut::with_capacity(12);
    unsafe {
        bytes_mut.set_len(4);
        bytes_mut.ptr = vptr(bytes_mut.ptr.as_ptr());
        bytes_mut.data = invalid_ptr((0 << ORIGINAL_CAPACITY_OFFSET) | KIND_VEC);
    }
    let additional = 10; // additional space is more than current capacity
    let allocate = false;

    let result = bytes_mut.reserve_inner(additional, allocate);
}

