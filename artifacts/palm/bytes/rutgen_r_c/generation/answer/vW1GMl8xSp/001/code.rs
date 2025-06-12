// Answer 0

#[test]
fn test_shared_v_is_unique_unique_ref_count() {
    use core::ptr::NonNull;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let shared = Box::new(Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 1024,
        ref_count: AtomicUsize::new(1),
    });

    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    unsafe {
        assert!(shared_v_is_unique(&data));
        ptr::drop_in_place(data.load(Ordering::Acquire).cast::<Shared>());
    }
}

#[test]
fn test_shared_v_is_unique_non_unique_ref_count() {
    use core::ptr::NonNull;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let shared = Box::new(Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 1024,
        ref_count: AtomicUsize::new(2),
    });

    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    unsafe {
        assert!(!shared_v_is_unique(&data));
        ptr::drop_in_place(data.load(Ordering::Acquire).cast::<Shared>());
    }
}

#[test]
#[should_panic]
fn test_shared_v_is_unique_invalid_pointer() {
    use core::ptr::NonNull;
    use std::sync::atomic::AtomicUsize;

    struct Shared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let invalid_data = AtomicPtr::new(ptr::null_mut());

    unsafe {
        shared_v_is_unique(&invalid_data);
    }
}

