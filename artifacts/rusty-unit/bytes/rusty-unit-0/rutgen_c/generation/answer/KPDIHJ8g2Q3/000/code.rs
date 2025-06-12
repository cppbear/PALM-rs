// Answer 0

#[test]
fn test_shared_v_clone() {
    use core::ptr;

    struct TestShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let shared_data = Box::new(TestShared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let atomic_ptr = AtomicPtr::new(Box::into_raw(shared_data) as *mut ());
    let bytes = unsafe {
        shared_v_clone(&atomic_ptr, [4, 5, 6].as_ptr(), 3)
    };

    assert_eq!(bytes.len, 3);
    assert_eq!(unsafe { ptr::read(bytes.ptr) }, 4);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn test_shared_v_clone_invalid_ref_count() {
    struct TestShared {
        vec: Vec<u8>,
        original_capacity_repr: usize,
        ref_count: AtomicUsize,
    }

    let shared_data = Box::new(TestShared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(isize::MAX as usize + 1),
    });

    let atomic_ptr = AtomicPtr::new(Box::into_raw(shared_data) as *mut ());
    unsafe {
        shared_v_clone(&atomic_ptr, [4, 5, 6].as_ptr(), 3);
    }
}

