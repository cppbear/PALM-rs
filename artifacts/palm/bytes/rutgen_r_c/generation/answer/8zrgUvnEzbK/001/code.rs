// Answer 0

#[test]
fn test_shared_to_mut_single_reference() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr;
    use std::sync::atomic::AtomicPtr;

    // Allocate memory for a shared buffer
    let layout = Layout::from_size_align(10, 1).unwrap();
    let buf = unsafe { alloc(layout) };
    
    // Initialize the shared structure
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    }));
    
    // Create an AtomicPtr pointing to the shared structure
    let data = AtomicPtr::new(shared.cast());
    
    // Initialize a buffer with values
    unsafe {
        ptr::write_bytes(buf, 1, 10); // Fill buffer with the byte value 1
    }
    
    // Call the function to test
    let result = unsafe { shared_to_mut(&data, buf, 10) };
    
    // Validate the result
    assert_eq!(result.len, 10);
    assert_eq!(unsafe { *(result.ptr.as_ptr()) }, 1);
    
    // Clean up
    unsafe {
        dealloc(buf, layout);
        let _ = Box::from_raw(shared); // This will deallocate shared structure
    }
}

#[test]
fn test_shared_to_mut_multiple_references() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr;
    use std::sync::atomic::AtomicPtr;

    // Allocate memory for a shared buffer
    let layout = Layout::from_size_align(10, 1).unwrap();
    let buf = unsafe { alloc(layout) };
    
    // Initialize the shared structure
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 10,
        ref_cnt: AtomicUsize::new(2), // Set ref count to 2 to trigger the alternate case
    }));
    
    // Create an AtomicPtr pointing to the shared structure
    let data = AtomicPtr::new(shared.cast());
    
    // Initialize a buffer with values
    unsafe {
        ptr::write_bytes(buf, 2, 10); // Fill buffer with the byte value 2
    }
    
    // Call the function to test
    let result = unsafe { shared_to_mut(&data, buf, 10) };
    
    // Validate the result
    assert_eq!(result.len, 10);
    assert_eq!(unsafe { *(result.ptr.as_ptr()) }, 2);

    // Clean up
    unsafe {
        dealloc(buf, layout);
        let _ = Box::from_raw(shared); // This will deallocate shared structure
    }
}

#[should_panic]
#[test]
fn test_shared_to_mut_zero_length() {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr;
    use std::sync::atomic::AtomicPtr;

    // Allocate memory for a shared buffer
    let layout = Layout::from_size_align(1, 1).unwrap();
    let buf = unsafe { alloc(layout) };
    
    // Initialize the shared structure
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 1,
        ref_cnt: AtomicUsize::new(1),
    }));
    
    // Create an AtomicPtr pointing to the shared structure
    let data = AtomicPtr::new(shared.cast());

    // Call the function with zero length
    let _ = unsafe { shared_to_mut(&data, buf, 0) };

    // Clean up
    unsafe {
        dealloc(buf, layout);
        let _ = Box::from_raw(shared); // This will deallocate shared structure
    }
}

