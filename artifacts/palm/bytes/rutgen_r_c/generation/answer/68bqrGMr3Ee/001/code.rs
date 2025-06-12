// Answer 0

#[test]
#[should_panic]
fn test_shallow_clone_arc_panic_on_reference_count_overflow() {
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut shared = MockShared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX >> 1), // Trigger overflow
    };

    let shared_ptr = &mut shared as *mut MockShared as *mut Shared;
    let ptr = std::ptr::null(); // Unsafe pointer
    let len = 0usize; // Zero-length for simplicity

    unsafe {
        shallow_clone_arc(shared_ptr, ptr, len);
    }
}

#[test]
fn test_shallow_clone_arc_successful_clone() {
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut shared = MockShared {
        buf: std::ptr::null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1), // Initial count
    };

    let shared_ptr = &mut shared as *mut MockShared as *mut Shared;
    let ptr = std::ptr::null(); // Unsafe pointer
    let len = 0usize; // Zero-length for simplicity

    let cloned_bytes: Bytes;

    unsafe {
        cloned_bytes = shallow_clone_arc(shared_ptr, ptr, len);
    }

    assert_eq!(cloned_bytes.len, len);
}

