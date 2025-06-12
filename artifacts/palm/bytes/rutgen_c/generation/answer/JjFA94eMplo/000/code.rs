// Answer 0

#[test]
fn test_shared_to_mut_impl_unique_handle() {
    use std::ptr;
    let shared_buf = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    }));

    let ptr = unsafe { (*shared_buf).buf };
    let len = 10;

    let result = unsafe { shared_to_mut_impl(shared_buf, ptr, len) };
    
    assert_eq!(result.len(), len);
    assert!(result.capacity() >= len);
}

#[test]
fn test_shared_to_mut_impl_multiple_handles() {
    use std::ptr;

    let shared_buf = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(2),
    }));

    let ptr = unsafe { (*shared_buf).buf };
    let len = 10;

    let result = unsafe { shared_to_mut_impl(shared_buf, ptr, len) };

    assert_eq!(result.len(), len);
    assert!(result.capacity() >= len);
}

#[test]
#[should_panic]
fn test_shared_to_mut_impl_invalid_ref_count() {
    use std::ptr;

    let shared_buf = Box::into_raw(Box::new(Shared {
        buf: ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(0),
    }));

    let ptr = ptr::null();
    let len = 0;

    unsafe { shared_to_mut_impl(shared_buf, ptr, len) };
}

