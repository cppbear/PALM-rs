// Answer 0

#[test]
fn test_promotable_is_unique_kind_vec() {
    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buffer: Vec<u8> = Vec::with_capacity(10);
    let shared = Box::into_raw(Box::new(DummyShared {
        buf: buffer.as_ptr() as *mut u8,
        cap: buffer.capacity(),
        ref_cnt: AtomicUsize::new(1),
    }));

    let atomic_ptr = AtomicPtr::new(shared as *mut ());

    unsafe {
        promotable_is_unique(&atomic_ptr);
    }

    // Clean up
    unsafe {
        let _ = Box::from_raw(shared);
    }
}

#[test]
fn test_promotable_is_unique_kind_vec_non_null() {
    struct DummyShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buffer: Vec<u8> = Vec::with_capacity(20);
    let shared = Box::into_raw(Box::new(DummyShared {
        buf: buffer.as_ptr() as *mut u8,
        cap: buffer.capacity(),
        ref_cnt: AtomicUsize::new(1),
    }));

    let atomic_ptr = AtomicPtr::new(shared as *mut ());

    unsafe {
        promotable_is_unique(&atomic_ptr);
    }

    // Clean up
    unsafe {
        let _ = Box::from_raw(shared);
    }
}

