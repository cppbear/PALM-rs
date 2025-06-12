// Answer 0

#[test]
fn test_release_shared_no_drop() {
    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let ptr = Box::into_raw(shared);
    
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_decrement() {
    let shared = Box::new(Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    });

    let ptr = Box::into_raw(shared);
    
    unsafe {
        release_shared(ptr);
    }
}

#[should_panic]
fn test_release_shared_invalid_pointer() {
    let ptr: *mut Shared = core::ptr::null_mut();
    
    unsafe {
        release_shared(ptr);
    }
}

