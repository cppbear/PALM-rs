// Answer 0

#[test]
fn test_shared_to_mut_impl_with_non_unique_handle() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr::null_mut;
    use std::mem::ManuallyDrop;
    use std::slice;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    struct BytesMut {
        vec: Vec<u8>,
    }
    
    impl BytesMut {
        fn from_vec(vec: Vec<u8>) -> Self {
            BytesMut { vec }
        }
        
        unsafe fn advance_unchecked(&mut self, _off: usize) {
            // Advance logic would be relevant; here we simulate as just a wrapper
            self.vec = self.vec.split_off(_off);
        }
    }

    unsafe fn release_shared(shared: *mut Shared) {
        // Simulated release logic
        if shared.is_null() { return; }
        let _ = Box::from_raw(shared);
    }

    let shared = Box::into_raw(Box::new(Shared {
        ref_cnt: AtomicUsize::new(2), // Set ref count to 2 to hit the non-unique handle case
        buf: Box::into_raw(Box::new([1u8, 2, 3, 4])) as *mut u8, // Simple buffer with some data
        cap: 4,
    }));

    let ptr = shared as *const Shared as *const u8; // Pointer for the buffer, simulating an access to shared data
    let len = 4; // Length to copy

    let b = unsafe { shared_to_mut_impl(shared, ptr, len) };

    assert_eq!(b.vec, vec![1, 2, 3, 4]); // Expecting copied bytes due to ref count > 1

    // Clean up memory management
    unsafe { release_shared(shared) };
}

#[test]
#[should_panic]
fn test_shared_to_mut_impl_panic_on_advance_unchecked() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::ptr::null_mut;
    use std::mem::ManuallyDrop;

    struct Shared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    struct BytesMut {
        vec: Vec<u8>,
    }
    
    impl BytesMut {
        fn from_vec(vec: Vec<u8>) -> Self {
            BytesMut { vec }
        }
        
        unsafe fn advance_unchecked(&mut self, _off: usize) {
            // Simulate panic condition
            panic!("Panic on advance_unchecked due to out of bounds access");
        }
    }

    unsafe fn release_shared(shared: *mut Shared) {
        if shared.is_null() { return; }
        let _ = Box::from_raw(shared);
    }

    let shared = Box::into_raw(Box::new(Shared {
        ref_cnt: AtomicUsize::new(1), // Ensure unique access
        buf: Box::into_raw(Box::new([1u8, 2, 3, 4])) as *mut u8,
        cap: 4,
    }));

    let ptr = shared as *const Shared as *const u8;
    let len = 0; // Edge case for advancing that would panic

    let _ = unsafe { shared_to_mut_impl(shared, ptr, len) };

    unsafe { release_shared(shared) };
}

