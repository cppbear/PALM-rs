// Answer 0

#[test]
fn test_release_shared_ref_count_one() {
    use core::ptr::null_mut;
    use alloc::boxed::Box;
    use core::sync::atomic::AtomicUsize;

    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let shared_memory = Box::new(Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    });

    let ptr: *mut Shared = Box::into_raw(shared_memory);
    
    unsafe {
        release_shared(ptr);
    }
}

#[test]
#[should_panic]
fn test_release_shared_invalid_ptr() {
    unsafe {
        release_shared(null_mut());
    }
}

