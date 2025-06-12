// Answer 0

#[test]
fn test_promotable_odd_to_mut() {
    use core::ptr::null_mut;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};

    // Prepare a valid configuration for the test
    let len = 10;
    let vec: Vec<u8> = (0..len).map(|i| i as u8).collect();
    let ptr = vec.as_ptr();
    
    // Create an AtomicPtr pointing to the vector
    let atomic_ptr = AtomicPtr::new(Box::into_raw(vec.into_boxed_slice()) as *mut ());

    // Call the function under test
    unsafe {
        let result = promotable_odd_to_mut(&atomic_ptr, ptr, len);
        
        // Check the properties of the returned BytesMut
        assert_eq!(result.len, len);
        assert!(result.cap >= len); // Ensure capacity is sufficient
        
        // Safety check: Ensure the pointer in BytesMut is valid
        let result_slice = slice::from_raw_parts(result.ptr.as_ptr(), result.len);
        assert_eq!(result_slice, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // Cleanup the allocated memory
        drop(Box::from_raw(atomic_ptr.load(Ordering::Relaxed) as *mut Vec<u8>));
    }
}

#[test]
#[should_panic]
fn test_promotable_odd_to_mut_invalid_len() {
    use core::ptr::null_mut;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};

    // Prepare an invalid configuration with length exceeding vector size
    let len = 20; // Exceeds the size of the vector
    let vec: Vec<u8> = (0..10).map(|i| i as u8).collect();
    let ptr = vec.as_ptr();

    // Create an AtomicPtr pointing to the vector
    let atomic_ptr = AtomicPtr::new(Box::into_raw(vec.into_boxed_slice()) as *mut ());

    // Call the function under test
    unsafe {
        promotable_odd_to_mut(&atomic_ptr, ptr, len); // This should panic
    }
}

