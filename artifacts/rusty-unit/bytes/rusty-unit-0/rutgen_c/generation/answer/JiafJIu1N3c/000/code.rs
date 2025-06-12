// Answer 0

#[test]
fn test_promotable_to_mut_kind_arc() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};
    
    struct Dummy;
    unsafe fn dummy_fn(_: *mut ()) -> *mut u8 {
        null_mut()
    }

    let data = AtomicPtr::new(&mut Dummy as *mut Dummy as *mut ());
    let ptr = null_mut();
    let len = 0;

    let result = unsafe { promotable_to_mut(&data, ptr, len, dummy_fn) };

    assert_eq!(result.len(), 0);
}

#[test]
fn test_promotable_to_mut_kind_vec() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};
    use alloc::vec::Vec;
    
    struct SharedBuffer {
        buf: *mut u8,
        cap: usize,
    }

    unsafe fn mock_alloc(_: *mut ()) -> *mut u8 {
        let vec = Vec::from_iter((0..10).map(|i| i as u8));
        let ptr = vec.as_mut_ptr();
        mem::forget(vec); // Prevent Vec from being dropped
        ptr
    }

    let shared = Box::into_raw(Box::new(SharedBuffer {
        buf: mock_alloc(null_mut()),
        cap: 10,
    })) as *mut ();

    let data = AtomicPtr::new(shared);
    let ptr = shared as *const SharedBuffer as *const u8;
    let len = 10;

    let result = unsafe { promotable_to_mut(&data, ptr, len, mock_alloc) };

    assert_eq!(result.len(), 10);
    assert_eq!(result.capacity(), 10);
}

