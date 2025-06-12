// Answer 0

#[test]
fn test_shallow_clone_arc() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr::null;

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    struct Bytes {
        ptr: *const u8,
        len: usize,
        data: *mut Shared,
        vtable: &'static str,
    }

    static SHARED_VTABLE: &str = "shared_vtable";

    unsafe fn shallow_clone_arc(shared: *mut Shared, ptr: *const u8, len: usize) -> Bytes {
        let old_size = (*shared).ref_cnt.fetch_add(1, Ordering::Relaxed);

        if old_size > usize::MAX >> 1 {
            std::process::abort();
        }

        Bytes {
            ptr,
            len,
            data: shared,
            vtable: &SHARED_VTABLE,
        }
    }

    let shared = Box::into_raw(Box::new(Shared {
        ref_cnt: AtomicUsize::new(0),
    }));

    let bytes = unsafe { shallow_clone_arc(shared, null(), 0) };

    assert_eq!(unsafe { (*shared).ref_cnt.load(Ordering::Relaxed) }, 1);
    assert_eq!(bytes.ptr, null());
    assert_eq!(bytes.len, 0);
    assert_eq!(bytes.vtable, &SHARED_VTABLE);

    // Cleanup
    unsafe {
        let _ = Box::from_raw(shared);
    }
}

#[test]
#[should_panic]
fn test_shallow_clone_arc_panic_on_ref_cnt() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    let shared = Box::new(Shared {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1), // Set to trigger panic
    });

    unsafe {
        shallow_clone_arc(Box::into_raw(shared), null(), 0);
    }
}

