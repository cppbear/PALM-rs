// Answer 0

#[test]
fn test_owned_clone_boundary_condition() {
    use core::ptr::null;
    use core::sync::atomic::AtomicPtr;

    // Set up a mock vtable to avoid undefined behavior
    static MOCK_VTABLE: Vtable = Vtable {
        clone: owned_clone,
        into_vec: unsafe { panic!() },
        into_mut: unsafe { panic!() },
        is_unique: unsafe { panic!() },
        drop: unsafe { panic!() },
    };

    // Create a mock OwnedLifetime with ref_cnt approaching half of usize::MAX
    struct MockOwnedLifetime {
        ref_cnt: AtomicUsize,
        // Drop function that does nothing
        drop: unsafe fn(*mut ()),
    }

    // Initialize the AtomicPtr with a valid reference for the OwnedLifetime instance
    let owned_lifetime = MockOwnedLifetime {
        ref_cnt: AtomicUsize::new((usize::MAX >> 1)),
        drop: unsafe { core::mem::transmute(0usize) },
    };
    
    let owned_ptr = AtomicPtr::new(&owned_lifetime as *const MockOwnedLifetime as *mut ());

    // Invoke the owned_clone function
    let result = unsafe { owned_clone(&owned_ptr, null(), 0) };

    // Validate results
    assert_eq!(result.len, 0);
    assert_eq!(result.ptr, null());
    assert_eq!(result.vtable, &MOCK_VTABLE);
    
    // Check the reference count is incremented correctly
    let ref_count = (&*owned_ptr.load(core::sync::atomic::Ordering::Relaxed).cast::<MockOwnedLifetime>()).ref_cnt.load(core::sync::atomic::Ordering::Relaxed);
    assert_eq!(ref_count, (usize::MAX >> 1) + 1);
}

