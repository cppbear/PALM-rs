// Answer 0

#[test]
fn test_owned_to_mut_valid() {
    use core::ptr;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};
    
    let mut data = Vec::new();
    data.push(1u8);
    data.push(2u8);
    data.push(3u8);

    let ptr = data.as_ptr();
    let len = data.len();
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    // Safety: The data is valid and correctly initialized
    let bytes_mut = unsafe { owned_to_mut(&atomic_ptr, ptr, len) };

    assert_eq!(bytes_mut.len(), len);
    assert!(!bytes_mut.is_empty());
}

#[test]
#[should_panic]
fn test_owned_to_mut_zero_length() {
    use core::ptr;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};

    let data: Vec<u8> = Vec::new();
    let ptr = data.as_ptr();
    let len = data.len();
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    // Safety: This should panic due to length being zero
    let _bytes_mut = unsafe { owned_to_mut(&atomic_ptr, ptr, len) };
}

#[test]
fn test_owned_to_mut_large_input() {
    use core::ptr;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};

    let mut data = Vec::with_capacity(1024);
    for i in 0..1024 {
        data.push(i as u8);
    }

    let ptr = data.as_ptr();
    let len = data.len();
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    // Safety: The data is valid and correctly initialized
    let bytes_mut = unsafe { owned_to_mut(&atomic_ptr, ptr, len) };

    assert_eq!(bytes_mut.len(), len);
    assert!(!bytes_mut.is_empty());
}

#[test]
fn test_owned_to_mut_empty_input() {
    use core::ptr;
    use alloc::vec::Vec;
    use core::sync::atomic::{AtomicPtr, Ordering};

    let data: Vec<u8> = Vec::new();
    let ptr = data.as_ptr();
    let len = data.len();
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    // Safety: Checking the behavior with empty data
    let bytes_mut = unsafe { owned_to_mut(&atomic_ptr, ptr, len) };

    assert_eq!(bytes_mut.len(), len);
    assert!(bytes_mut.is_empty());
}

