// Answer 0

#[test]
fn test_promotable_to_vec_kinds_check() {
    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }
    
    let mut data: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());
    let len = 10;
    
    let mut vec_buf = vec![0u8; len];
    let mut dummy_shared = DummyShared {
        buf: vec_buf.as_mut_ptr(),
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    };

    // Store a pointer to DummyShared as an AtomicPtr
    let shared_ptr = &mut dummy_shared as *mut DummyShared as *mut ();
    data.store(shared_ptr, Ordering::Release);
    
    // Function pointer to allocate memory for the buffer
    let alloc_fn = |shared: *mut ()| -> *mut u8 {
        unsafe {
            let shared_ref = &*(shared as *mut DummyShared);
            shared_ref.buf
        }
    };
    
    // Should execute without panicking as the kind is KIND_VEC
    let result_vec = unsafe {
        promotable_to_vec(&data, dummy_shared.buf, len, alloc_fn)
    };

    // Ensure the content of the vector is same as initial buffer
    assert_eq!(result_vec, vec![0; len]);
}

#[test]
fn test_promotable_to_vec_when_panic() {
    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }
    
    let mut data: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());
    let len = 10;

    let initial_buf = vec![1u8; len];
    let mut vec_buf = vec![0u8; len];
    let mut dummy_shared = DummyShared {
        buf: vec_buf.as_mut_ptr(),
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    };

    // Store a pointer to DummyShared as an AtomicPtr
    let shared_ptr = &mut dummy_shared as *mut DummyShared as *mut ();
    data.store(shared_ptr, Ordering::Release);
    
    // Function pointer to allocate memory for the buffer
    let alloc_fn = |shared: *mut ()| -> *mut u8 {
        unsafe {
            let shared_ref = &*(shared as *mut DummyShared);
            shared_ref.buf
        }
    };

    // Change ref_count to trigger panic (not necessary for this function)
    dummy_shared.ref_cnt.store(2, Ordering::Release);

    // Check that calling the function with unexpected kind will trigger a panic
    #[should_panic(expected = "assertion failed: kind == KIND_VEC")]
    unsafe {
        let _ = promotable_to_vec(&data, initial_buf.as_ptr(), len, alloc_fn);
    }
}

