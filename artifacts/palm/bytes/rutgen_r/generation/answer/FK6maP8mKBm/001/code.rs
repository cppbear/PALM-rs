// Answer 0

#[test]
fn test_shared_to_vec_impl_unique_ref_count() {
    use std::ptr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::mem::ManuallyDrop;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    unsafe fn create_shared(cap: usize) -> (*mut Shared, *mut u8) {
        let buf = Box::new(vec![0u8; cap]).into_raw();
        let shared = Box::new(Shared {
            ref_cnt: AtomicUsize::new(1),
            buf,
            cap,
        });
        (Box::into_raw(shared), buf)
    }

    let cap = 10;
    let (shared_ptr, buf_ptr) = create_shared(cap);

    let len = cap;
    let data = vec![1u8; len]; // Data to copy into the shared buffer

    let result = unsafe { shared_to_vec_impl(shared_ptr, data.as_ptr(), len) };

    // Verify the result matches the expected output
    assert_eq!(result, data);

    // Verify the buffer contents after the copy
    let buf = ManuallyDrop::new(*Box::from_raw(shared_ptr));
    let slice = std::slice::from_raw_parts(buf.buf, len);
    assert_eq!(slice, &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

    // Clean up the allocated buffer since we've manually deallocated it
    unsafe { Box::from_raw(buf.buf) };
}

#[test]
#[should_panic]
fn test_shared_to_vec_impl_non_unique_ref_count() {
    use std::ptr;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    unsafe fn create_shared(cap: usize) -> (*mut Shared, *mut u8) {
        let buf = Box::new(vec![0u8; cap]).into_raw();
        let shared = Box::new(Shared {
            ref_cnt: AtomicUsize::new(2),
            buf,
            cap,
        });
        (Box::into_raw(shared), buf)
    }

    let cap = 10;
    let (shared_ptr, buf_ptr) = create_shared(cap);

    let len = cap;
    let data = vec![1u8; len]; // Data to copy into the shared buffer

    // Attempt to invoke shared_to_vec_impl with a non-unique ref count
    // This should trigger release_shared and thus should panic upon invalid operation
    unsafe {
        let _ = shared_to_vec_impl(shared_ptr, data.as_ptr(), len);
    }
}

