// Answer 0

#[test]
fn test_release_shared_ref_count_is_zero() {
    use core::ptr;
    use alloc::boxed::Box;
    use core::sync::atomic::AtomicUsize;

    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mock_ptr: *mut MockShared = Box::into_raw(Box::new(MockShared {
        buf: ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    }));

    unsafe {
        release_shared(mock_ptr);
    }
}

#[test]
#[should_panic]
fn test_release_shared_ref_count_not_zero() {
    use core::ptr;
    use alloc::boxed::Box;
    use core::sync::atomic::AtomicUsize;

    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mock_ptr: *mut MockShared = Box::into_raw(Box::new(MockShared {
        buf: ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(2),
    }));

    unsafe {
        release_shared(mock_ptr);
        release_shared(mock_ptr);
    }
}

