// Answer 0

#[test]
fn test_owned_clone_max_ref_count() {
    unsafe {
        let mut ref_count = 0;
        let owned_lifetime = OwnedLifetime {
            ref_cnt: AtomicUsize::new(ref_count),
            drop: std::mem::transmute::<fn(*mut ()), *mut ()>(std::mem::drop),
        };
        let owned_ptr = Box::into_raw(Box::new(owned_lifetime));
        let data = AtomicPtr::new(owned_ptr as *mut ());

        // Set old_cnt to usize::MAX >> 1
        ref_count = usize::MAX >> 1;
        data.load(Ordering::Relaxed); // Simulate loading the ptr

        let ptr: *const u8 = std::ptr::null();
        let len: usize = 0;

        // Perform the clone operation which should not panic
        let _result = owned_clone(&data, ptr, len);
    }
}

#[test]
fn test_owned_clone_zero_length() {
    unsafe {
        let mut ref_count = 0;
        let owned_lifetime = OwnedLifetime {
            ref_cnt: AtomicUsize::new(ref_count),
            drop: std::mem::transmute::<fn(*mut ()), *mut ()>(std::mem::drop),
        };
        let owned_ptr = Box::into_raw(Box::new(owned_lifetime));
        let data = AtomicPtr::new(owned_ptr as *mut ());

        // Set old_cnt to usize::MAX >> 1
        ref_count = usize::MAX >> 1;
        data.load(Ordering::Relaxed); // Simulate loading the ptr

        let ptr: *const u8 = std::ptr::null();
        let len: usize = 0;

        // Perform the clone operation with zero length
        let _result = owned_clone(&data, ptr, len);
    }
}

#[test]
fn test_owned_clone_large_length() {
    unsafe {
        let mut ref_count = 0;
        let owned_lifetime = OwnedLifetime {
            ref_cnt: AtomicUsize::new(ref_count),
            drop: std::mem::transmute::<fn(*mut ()), *mut ()>(std::mem::drop),
        };
        let owned_ptr = Box::into_raw(Box::new(owned_lifetime));
        let data = AtomicPtr::new(owned_ptr as *mut ());

        // Set old_cnt to usize::MAX >> 1
        ref_count = usize::MAX >> 1;
        data.load(Ordering::Relaxed); // Simulate loading the ptr

        let ptr: *const u8 = std::ptr::null();
        let len: usize = usize::MAX; // Test with the maximum length

        // Perform the clone operation which should not panic
        let _result = owned_clone(&data, ptr, len);
    }
}

