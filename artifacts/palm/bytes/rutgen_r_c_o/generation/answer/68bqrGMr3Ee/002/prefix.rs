// Answer 0

#[test]
fn test_shallow_clone_arc_valid_pointer_with_half_max_old_size() {
    use core::ptr;

    #[repr(C)]
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut mock_shared = MockShared {
        buf: ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
    };

    let shared_ptr = &mut mock_shared as *mut MockShared as *mut Shared;

    let ptr: *const u8 = ptr::null();
    let len: usize = 0;

    unsafe {
        shallow_clone_arc(shared_ptr, ptr, len);
    }
}

#[test]
fn test_shallow_clone_arc_valid_pointer_with_non_zero_len() {
    use core::ptr;

    #[repr(C)]
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut mock_shared = MockShared {
        buf: ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
    };

    let shared_ptr = &mut mock_shared as *mut MockShared as *mut Shared;

    let data: [u8; 5] = [1, 2, 3, 4, 5];
    let ptr: *const u8 = data.as_ptr();
    let len: usize = data.len();

    unsafe {
        shallow_clone_arc(shared_ptr, ptr, len);
    }
}

#[test]
#[should_panic]
fn test_shallow_clone_arc_exceeding_ref_count() {
    use core::ptr;

    #[repr(C)]
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut mock_shared = MockShared {
        buf: ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX), // Set to max to trigger panic
    };

    let shared_ptr = &mut mock_shared as *mut MockShared as *mut Shared;

    let ptr: *const u8 = ptr::null();
    let len: usize = 0;

    unsafe {
        shallow_clone_arc(shared_ptr, ptr, len);
    }
}

