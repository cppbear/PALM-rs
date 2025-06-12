// Answer 0

#[test]
fn test_promotable_even_clone_with_kind_vec() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};
    use alloc::boxed::Box;

    struct Dummy;

    // Create a shared buffer
    let buffer: *mut u8 = Box::into_raw(Box::new([0u8; 10])) as *mut u8;
    let len: usize = 5;
    let atomic_ptr = AtomicPtr::new(buffer as _);
    
    // Setting kind to KIND_VEC
    let kind_vec: *mut Shared = Box::into_raw(Box::new(Shared {
        buf: buffer,
        cap: len,
        ref_cnt: AtomicUsize::new(1),
    }));
    
    // Mask to represent it as KIND_VEC
    let kind_masked = (kind_vec as usize & !KIND_MASK) as *mut ();
    
    // Store the shared buffer into AtomicPtr
    atomic_ptr.store(kind_masked, Ordering::Release);

    // Call the function with kind set to KIND_VEC
    let clone = unsafe { promotable_even_clone(&atomic_ptr, null_mut(), len) };

    // Assert the clone values
    assert_eq!(clone.len, len);
    assert_eq!(clone.ptr, null_mut()); // since we passed null_mut() as ptr

    // Clean up to prevent memory leaks
    unsafe {
        Box::from_raw(kind_vec);
    }
}

#[test]
#[should_panic]
fn test_promotable_even_clone_panic_on_invalid_condition() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};

    // Create a shared buffer, but with incorrect setup to provoke panic
    let atomic_ptr = AtomicPtr::new(null_mut());
    let len: usize = 5;

    // This should panic because we have an invalid pointer
    unsafe {
        let _ = promotable_even_clone(&atomic_ptr, null_mut(), len);
    }
}

