// Answer 0

#[test]
fn test_shallow_clone_arc_valid() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr::null;

    struct Shared {
        ref_cnt: AtomicUsize,
    }

    struct Bytes {
        ptr: *const u8,
        len: usize,
        data: AtomicPtr<Shared>,
        vtable: &'static u32, // Example placeholder for the vtable
    }

    const SHARED_VTABLE: u32 = 0;

    unsafe fn shallow_clone_arc(shared: *mut Shared, ptr: *const u8, len: usize) -> Bytes {
        let old_size = (*shared).ref_cnt.fetch_add(1, Ordering::Relaxed);

        if old_size > usize::MAX >> 1 {
            crate::abort();
        }

        Bytes {
            ptr,
            len,
            data: AtomicPtr::new(shared as _),
            vtable: &SHARED_VTABLE,
        }
    }

    // Setup the test input
    let shared_count = usize::MAX >> 1; // Set the initial reference count to MAX / 2
    let shared = Box::into_raw(Box::new(Shared { ref_cnt: AtomicUsize::new(shared_count) }));
    let ptr: *const u8 = null();
    let len = 0;

    // Call the function under test
    let result = unsafe { shallow_clone_arc(shared, ptr, len) };

    // Validate the result
    assert_eq!(result.ptr, ptr);
    assert_eq!(result.len, len);
    assert_eq!(result.data.load(Ordering::SeqCst), shared as _);
    assert_eq!(result.vtable, &SHARED_VTABLE);

    // Clean up
    let _ = Box::from_raw(shared);
}

