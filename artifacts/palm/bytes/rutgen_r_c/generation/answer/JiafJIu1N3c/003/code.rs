// Answer 0

#[test]
fn test_promotable_to_mut_kind_vec() {
    use core::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct DummyShared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    // Create a dummy buffer to simulate a Vec backing storage.
    let capacity = 10;
    let vec = vec![1u8; capacity];
    let buffer = vec.as_ptr() as *mut u8;

    // Create a shared structure with KIND_VEC
    let shared = Box::into_raw(Box::new(DummyShared {
        ref_cnt: AtomicUsize::new(1),
        buf: buffer,
        cap: capacity,
    }));

    // Using AtomicPtr to simulate behavior in the function.
    let atomic_ptr = AtomicPtr::new(shared as *mut ());

    // This is a dummy function that returns the original buffer for KIND_VEC.
    let dummy_fn: fn(*mut ()) -> *mut u8 = |ptr| {
        let shared: *mut DummyShared = ptr.cast();
        unsafe { (*shared).buf }
    };

    // Call the function
    let result = unsafe { promotable_to_mut(&atomic_ptr, buffer, capacity, dummy_fn) };

    // Check the conditions:
    // 1. The length of result should equal the capacity.
    // 2. The values in result should match the original values in the Vec used for the buffer.
    assert_eq!(result.len(), capacity);
    for i in 0..capacity {
        assert_eq!(result.as_slice()[i], 1);
    }

    // Clean up the memory to prevent leaks
    unsafe {
        let _ = Box::from_raw(shared);
    }
}

#[test]
#[should_panic(expected = "internal: set_start out of bounds")]
fn test_promotable_to_mut_out_of_bounds() {
    use core::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct DummyShared {
        ref_cnt: AtomicUsize,
        buf: *mut u8,
        cap: usize,
    }

    let capacity = 10;
    let vec = vec![1u8; capacity];
    let buffer = vec.as_ptr() as *mut u8;

    let shared = Box::into_raw(Box::new(DummyShared {
        ref_cnt: AtomicUsize::new(1),
        buf: buffer,
        cap: capacity,
    }));

    let atomic_ptr = AtomicPtr::new(shared as *mut ());

    // This function returns a pointer to a buffer that is out of bounds
    let dummy_fn: fn(*mut ()) -> *mut u8 = |_| {
        null_mut() // Returning null for testing panic condition
    };

    // Trigger the panic due to out of bounds access
    unsafe {
        promotable_to_mut(&atomic_ptr, buffer, capacity, dummy_fn);
    }
}

