// Answer 0

#[test]
fn test_shared_clone_success() {
    use core::ptr::NonNull;
    use std::mem;

    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut dummy_shared = DummyShared {
        buf: Box::into_raw(Box::new([0u8; 10])),
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    };

    let atomic_ptr = AtomicPtr::new(&mut dummy_shared as *mut _ as *mut ());
    let ptr = dummy_shared.buf;
    let len = dummy_shared.cap;

    // Ensuring the function doesn't cause a panic and returns a valid Bytes instance
    let bytes = unsafe {
        shared_clone(&atomic_ptr, ptr, len)
    };

    assert_eq!(bytes.len, len);
    assert_eq!(bytes.ptr, ptr);
    assert!(!bytes.data.load(Ordering::Relaxed).is_null());
    assert_eq!(bytes.vtable, &SHARED_VTABLE);

    // Clean up
    unsafe {
        drop(Box::from_raw(dummy_shared.buf));
        let shared_ptr = bytes.data.load(Ordering::Relaxed);
        drop(Box::from_raw(shared_ptr as *mut Shared));
    }
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_shared_clone_overflow() {
    use core::ptr::NonNull;
    use std::mem;

    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut dummy_shared = DummyShared {
        buf: Box::into_raw(Box::new([0u8; 10])),
        cap: 10,
        ref_cnt: AtomicUsize::new(usize::MAX), // Set to MAX to force overflow
    };

    let atomic_ptr = AtomicPtr::new(&mut dummy_shared as *mut _ as *mut ());
    let ptr = dummy_shared.buf;
    let len = dummy_shared.cap;

   // This should panic due to attempted overflow of ref_cnt
    unsafe {
        shared_clone(&atomic_ptr, ptr, len);
    }
}

