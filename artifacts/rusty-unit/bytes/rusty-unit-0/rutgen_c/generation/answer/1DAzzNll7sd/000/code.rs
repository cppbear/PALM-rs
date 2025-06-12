// Answer 0

#[test]
fn test_shared_clone() {
    use core::ptr::null_mut;
    use core::sync::atomic::AtomicPtr;

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let shared = Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1), // Initial reference count set to 1
    };

    let atomic_ptr = AtomicPtr::new(&shared as *const Shared as *mut Shared);
    let ptr: *const u8 = null_mut(); // Example pointer, doesn't point to valid data
    let len: usize = 0; // Example length

    let cloned_bytes = unsafe { shared_clone(&atomic_ptr, ptr, len) };

    assert_eq!(cloned_bytes.ptr, ptr);
    assert_eq!(cloned_bytes.len, len);
    assert_eq!(cloned_bytes.data.load(core::sync::atomic::Ordering::Relaxed), &shared as *const Shared as *mut Shared);
}

#[test]
#[should_panic]
fn test_shared_clone_exceeding_ref_count() {
    use core::ptr::null_mut;
    use core::sync::atomic::AtomicPtr;

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let mut shared = Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(usize::MAX), // Set to maximum to trigger panic
    };

    let atomic_ptr = AtomicPtr::new(&mut shared as *mut Shared);
    let ptr: *const u8 = null_mut(); // Example pointer, doesn't point to valid data
    let len: usize = 0; // Example length

    // This will panic when attempting to clone due to reference count exceeding limit
    let _ = unsafe { shared_clone(&atomic_ptr, ptr, len) };
}

