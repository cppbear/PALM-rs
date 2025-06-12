// Answer 0

#[test]
fn test_shared_to_mut_impl_single_ref() {
    use std::sync::{Arc, Mutex};
    use std::ptr::null_mut;
    use std::mem::ManuallyDrop;
    use std::slice;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use bytes::BytesMut;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    unsafe {
        let shared = Box::new(Shared {
            ref_cnt: AtomicUsize::new(1),
            buf: Box::into_raw(Box::new([1, 2, 3, 4, 5])) as *mut u8,
            cap: 5,
        });

        let ptr = shared.buf;
        let len = 5;

        let result = shared_to_mut_impl(Box::into_raw(shared), ptr, len);
        let result_vec: Vec<u8> = result.to_vec();

        assert_eq!(result_vec, vec![1, 2, 3, 4, 5]);

        // Ensure buffer is deallocated
        let _ = Box::from_raw(ptr);
    }
}

#[test]
fn test_shared_to_mut_impl_multiple_refs() {
    use std::sync::{Arc, Mutex};
    use std::ptr::null_mut;
    use std::slice;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use bytes::BytesMut;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    unsafe {
        let shared = Box::new(Shared {
            ref_cnt: AtomicUsize::new(2),
            buf: Box::into_raw(Box::new([6, 7, 8, 9, 10])) as *mut u8,
            cap: 5,
        });

        let ptr = shared.buf;
        let len = 5;

        let result = shared_to_mut_impl(Box::into_raw(shared), ptr, len);
        let result_vec: Vec<u8> = result.to_vec();

        assert_eq!(result_vec, vec![6, 7, 8, 9, 10]);

        // Ensure buffer is deallocated
        let _ = Box::from_raw(ptr);
    }
}

