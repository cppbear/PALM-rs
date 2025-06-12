// Answer 0

#[test]
fn test_static_to_vec_valid_input() {
    let data: [u8; 5] = [1, 2, 3, 4, 5];
    let layout = Layout::from_size_align(5, 1).unwrap();
    let ptr = unsafe { alloc::alloc(layout) };
    unsafe {
        ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
        let len = 5;
        let result = static_to_vec(&AtomicPtr::new(ptr), ptr, len);
        dealloc(ptr, layout);
    }
}

#[test]
fn test_static_to_vec_zero_length() {
    let ptr: *const u8 = &0u8 as *const u8;
    let len = 0;
    let result = static_to_vec(&AtomicPtr::new(ptr as *mut ()), ptr, len);
}

#[test]
fn test_static_to_vec_max_length() {
    let len = (1 << 30) - 1;
    let layout = Layout::from_size_align(len, 1).unwrap();
    let ptr = unsafe { alloc::alloc(layout) };
    if !ptr.is_null() {
        unsafe {
            let result = static_to_vec(&AtomicPtr::new(ptr), ptr, len);
            dealloc(ptr, layout);
        }
    }
}

#[should_panic]
fn test_static_to_vec_null_pointer() {
    let ptr: *const u8 = std::ptr::null();
    let len = 1;
    let _result = static_to_vec(&AtomicPtr::new(ptr as *mut ()), ptr, len);
}

#[test]
fn test_static_to_vec_exceeding_length() {
    let data: [u8; 5] = [1, 2, 3, 4, 5];
    let layout = Layout::from_size_align(5, 1).unwrap();
    let ptr = unsafe { alloc::alloc(layout) };
    unsafe {
        ptr.copy_from_nonoverlapping(data.as_ptr(), data.len());
        let len = 6; // Intentional invalid length
        let result = static_to_vec(&AtomicPtr::new(ptr), ptr, len);
        dealloc(ptr, layout);
    }
}

