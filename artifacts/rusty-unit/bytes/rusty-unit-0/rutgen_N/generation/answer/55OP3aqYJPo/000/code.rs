// Answer 0

#[test]
fn test_promotable_even_to_vec() {
    use std::ptr::AtomicPtr;
    use std::ffi::c_void;

    const KIND_MASK: usize = 0b11;

    unsafe fn promotable_to_vec<F>(data: &AtomicPtr<()>, ptr: *const u8, len: usize, map_fn: F) -> Vec<u8>
    where
        F: Fn(*const c_void) -> *const c_void,
    {
        let mapped_ptr = map_fn(data.load(std::sync::atomic::Ordering::Relaxed).cast());
        Vec::from_raw_parts(mapped_ptr as *mut u8, len, len)
    }

    unsafe fn ptr_map(shared: *const c_void, mapper: fn(*const c_void) -> *const c_void) -> *const c_void {
        mapper(shared)
    }

    let data = AtomicPtr::new(std::ptr::null_mut());
    let test_ptr: *const u8 = [1, 2, 3, 4].as_ptr();
    let len: usize = 4;

    let result_vec = promotable_even_to_vec(&data, test_ptr, len);

    assert_eq!(result_vec, vec![1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_promotable_even_to_vec_with_zero_length() {
    use std::ptr::AtomicPtr;
    use std::ffi::c_void;

    const KIND_MASK: usize = 0b11;

    unsafe fn promotable_to_vec<F>(data: &AtomicPtr<()>, ptr: *const u8, len: usize, map_fn: F) -> Vec<u8>
    where
        F: Fn(*const c_void) -> *const c_void,
    {
        let mapped_ptr = map_fn(data.load(std::sync::atomic::Ordering::Relaxed).cast());
        Vec::from_raw_parts(mapped_ptr as *mut u8, len, len)
    }

    unsafe fn ptr_map(shared: *const c_void, mapper: fn(*const c_void) -> *const c_void) -> *const c_void {
        mapper(shared)
    }

    let data = AtomicPtr::new(std::ptr::null_mut());
    let test_ptr: *const u8 = [1, 2, 3, 4].as_ptr();
    let len: usize = 0;

    let _result_vec = promotable_even_to_vec(&data, test_ptr, len);
}

