// Answer 0

#[test]
fn test_owned_to_mut_valid() {
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use bytes::BytesMut;

    // Setup the AtomicPtr with a valid memory allocation
    let data: AtomicPtr<()> = AtomicPtr::new(unsafe { libc::malloc(10) as *mut () }); // Allocate 10 bytes
    let ptr: *const u8 = data.load(Ordering::SeqCst) as *const u8;
    let len: usize = 10;

    // Ensure the function operates as expected
    let result = unsafe { owned_to_mut(&data, ptr, len) };

    assert_eq!(result.len(), len);
    
    // Free the allocated memory
    unsafe { libc::free(data.load(Ordering::SeqCst) as *mut libc::c_void) };
}

#[should_panic]
#[test]
fn test_owned_to_mut_null_pointer() {
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use bytes::BytesMut;

    let data: AtomicPtr<()> = AtomicPtr::new(null_mut());
    let ptr: *const u8 = null();
    let len: usize = 0;

    // This should panic as we are passing a null pointer
    let _ = unsafe { owned_to_mut(&data, ptr, len) };
}

#[should_panic]
#[test]
fn test_owned_to_mut_negative_length() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use bytes::BytesMut;

    let data: AtomicPtr<()> = AtomicPtr::new(unsafe { libc::malloc(10) as *mut () }); // Allocate 10 bytes
    let ptr: *const u8 = data.load(Ordering::SeqCst) as *const u8;
    let len: usize = usize::MAX; // This will be treated as an invalid length

    // This should panic due to invalid length
    let _ = unsafe { owned_to_mut(&data, ptr, len) };

    // Free the allocated memory
    unsafe { libc::free(data.load(Ordering::SeqCst) as *mut libc::c_void) };
}

