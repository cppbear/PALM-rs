// Answer 0

#[test]
fn test_promotable_even_clone_arc_valid() {
    let shared_data = Box::new(Shared {
        buf: Box::into_raw(Box::new(0u8)),
        cap: 1,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_data as *mut _);
    let ptr = Box::into_raw(Box::new(1u8));
    let len = 1;

    unsafe {
        let _result = promotable_even_clone(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_promotable_even_clone_arc_zero_length() {
    let shared_data = Box::new(Shared {
        buf: Box::into_raw(Box::new(0u8)),
        cap: 1,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_data as *mut _);
    let ptr = Box::into_raw(Box::new(1u8));
    let len = 0;

    unsafe {
        let _result = promotable_even_clone(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_promotable_even_clone_arc_max_length() {
    let shared_data = Box::new(Shared {
        buf: Box::into_raw(Box::new(0u8)),
        cap: 1,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_data as *mut _);
    let ptr = Box::into_raw(Box::new(1u8));
    let len = usize::MAX;

    unsafe {
        let _result = promotable_even_clone(&atomic_ptr, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_promotable_even_clone_arc_invalid_shared() {
    let atomic_ptr = AtomicPtr::new(0 as *mut _);
    let ptr = Box::into_raw(Box::new(1u8));
    let len = 1;

    unsafe {
        let _result = promotable_even_clone(&atomic_ptr, ptr, len);
    }
}

