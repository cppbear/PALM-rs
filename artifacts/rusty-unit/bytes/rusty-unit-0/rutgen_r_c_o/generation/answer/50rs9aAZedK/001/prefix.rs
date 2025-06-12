// Answer 0

#[test]
fn test_owned_to_vec_empty() {
    let data = AtomicPtr::new(core::ptr::null_mut());
    let ptr: *const u8 = core::ptr::null();
    let len: usize = 0;
    unsafe {
        owned_to_vec(&data, ptr, len);
    }
}

#[test]
fn test_owned_to_vec_non_empty() {
    let data = AtomicPtr::new(core::ptr::null_mut());
    let sample_data: &[u8] = &[1, 2, 3, 4, 5];
    let ptr: *const u8 = sample_data.as_ptr();
    let len: usize = sample_data.len();
    unsafe {
        owned_to_vec(&data, ptr, len);
    }
}

#[should_panic]
fn test_owned_to_vec_invalid_length() {
    let data = AtomicPtr::new(core::ptr::null_mut());
    let sample_data: &[u8] = &[1, 2, 3, 4, 5];
    let ptr: *const u8 = sample_data.as_ptr();
    let len: usize = sample_data.len() + 1; // Out of bounds
    unsafe {
        owned_to_vec(&data, ptr, len);
    }
}

#[should_panic]
fn test_owned_to_vec_large_length() {
    let data = AtomicPtr::new(core::ptr::null_mut());
    let ptr: *const u8 = core::ptr::null(); // Valid, but will trigger panic on length
    let len: usize = usize::MAX; // Panic on max size
    unsafe {
        owned_to_vec(&data, ptr, len);
    }
}

