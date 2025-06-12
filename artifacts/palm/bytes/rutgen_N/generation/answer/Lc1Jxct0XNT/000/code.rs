// Answer 0

#[test]
fn test_shared_v_to_mut_unique() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::mem;
    use std::slice;
    use std::ptr;

    struct Shared {
        vec: Vec<u8>,
    }

    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl BytesMut {
        fn from_vec(vec: Vec<u8>) -> Self {
            let len = vec.len();
            let cap = vec.capacity();
            let ptr = vec.as_mut_ptr();
            mem::forget(vec); // Prevent dropping the Vec
            BytesMut { ptr, len, cap, data: ptr::null_mut() }
        }
    }

    unsafe fn offset_from(ptr: *mut u8, base: *mut u8) -> usize {
        (ptr as usize).wrapping_sub(base as usize)
    }

    unsafe fn release_shared(shared: *mut Shared) {
        // Logic to release shared memory (stub for the test)
    }

    unsafe fn vptr(ptr: *mut u8) -> *mut u8 {
        ptr
    }

    // Initialize shared memory data
    let vec = vec![1, 2, 3, 4, 5];
    let shared = Box::into_raw(Box::new(Shared { vec }));
    let data = AtomicPtr::new(shared as *mut _);
    
    let ptr = data.load(Ordering::Relaxed).cast::<Shared>().as_ref().unwrap().vec.as_ptr();
    let len = 5;

    let bytes_mut = unsafe { shared_v_to_mut(&data, ptr, len) };

    assert_eq!(bytes_mut.len, len);
    assert_eq!(bytes_mut.cap, len);
    assert!(!bytes_mut.ptr.is_null());
    assert_eq!(unsafe { slice::from_raw_parts(bytes_mut.ptr, bytes_mut.len) }, [1, 2, 3, 4, 5]);

    unsafe { release_shared(shared) };
}

#[test]
fn test_shared_v_to_mut_non_unique() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::mem;
    use std::slice;
    use std::ptr;

    struct Shared {
        vec: Vec<u8>,
    }

    struct BytesMut {
        ptr: *mut u8,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl BytesMut {
        fn from_vec(vec: Vec<u8>) -> Self {
            let len = vec.len();
            let cap = vec.capacity();
            let ptr = vec.as_mut_ptr();
            mem::forget(vec); // Prevent dropping the Vec
            BytesMut { ptr, len, cap, data: ptr::null_mut() }
        }
    }

    unsafe fn offset_from(ptr: *mut u8, base: *mut u8) -> usize {
        (ptr as usize).wrapping_sub(base as usize)
    }

    unsafe fn release_shared(shared: *mut Shared) {
        // Logic to release shared memory (stub for the test)
    }

    unsafe fn vptr(ptr: *mut u8) -> *mut u8 {
        ptr
    }

    // Initialize shared memory data
    let vec = vec![1, 2, 3, 4, 5];
    let shared_ptr = Box::into_raw(Box::new(Shared { vec }));
    let data = AtomicPtr::new(shared_ptr as *mut _);

    // Simulate non-unique case by not freeing it (could be mocked or handled based on your context).
    // For simplicity, we are using the same shared pointer here directly.
    let ptr = data.load(Ordering::Relaxed).cast::<Shared>().as_ref().unwrap().vec.as_ptr();
    let len = 5;

    let bytes_mut = unsafe { shared_v_to_mut(&data, ptr, len) };

    assert_eq!(bytes_mut.len, len);
    assert!(bytes_mut.cap >= len); // Capacity could be more than len, verify boundary case.
    assert!(!bytes_mut.ptr.is_null());
    assert_eq!(unsafe { slice::from_raw_parts(bytes_mut.ptr, bytes_mut.len) }, [1, 2, 3, 4, 5]);

    unsafe { release_shared(shared_ptr) }; // Release after test
}

