// Answer 0

#[test]
fn test_with_vtable_valid_data() {
    use core::ptr::NonNull;
    use alloc::alloc::Layout;
    use std::sync::atomic::AtomicPtr;

    let data_ptr = Box::into_raw(Box::new(42)) as *const u8; // Valid pointer
    let length = 4; // Length of pointer
    let atomic_data = AtomicPtr::new(data_ptr as *mut ());
    
    let vtable = &STATIC_VTABLE; // Using the static vtable
    let bytes = unsafe { Bytes::with_vtable(data_ptr, length, atomic_data, vtable) };
    
    assert_eq!(bytes.ptr, data_ptr); // Validate pointer
    assert_eq!(bytes.len, length); // Validate length
    assert_eq!(bytes.data.load(std::sync::atomic::Ordering::SeqCst), data_ptr as *mut ()); // Validate atomic data
    assert_eq!(bytes.vtable, vtable); // Validate vtable
}

#[test]
#[should_panic]
fn test_with_vtable_null_pointer() {
    use std::sync::atomic::AtomicPtr;

    let null_ptr: *const u8 = core::ptr::null();
    let length = 0; 
    let atomic_data = AtomicPtr::new(null_ptr as *mut ());
    
    let vtable = &STATIC_VTABLE; 
    unsafe { Bytes::with_vtable(null_ptr, length, atomic_data, vtable) }; // Should panic due to null pointer
}

#[test]
fn test_with_vtable_zero_length() {
    use core::ptr::NonNull;
    use alloc::alloc::Layout;
    use std::sync::atomic::AtomicPtr;

    let data_ptr = Box::into_raw(Box::new(42)) as *const u8; 
    let length = 0; // Zero length should be valid
    let atomic_data = AtomicPtr::new(data_ptr as *mut ());
    
    let vtable = &STATIC_VTABLE; 
    let bytes = unsafe { Bytes::with_vtable(data_ptr, length, atomic_data, vtable) };

    assert_eq!(bytes.ptr, data_ptr); // Validate pointer
    assert_eq!(bytes.len, length); // Validate length
    assert_eq!(bytes.data.load(std::sync::atomic::Ordering::SeqCst), data_ptr as *mut ()); // Validate atomic data
    assert_eq!(bytes.vtable, vtable); // Validate vtable
}

#[test]
fn test_with_vtable_large_length() {
    use core::ptr::NonNull;
    use alloc::alloc::Layout;
    use std::sync::atomic::AtomicPtr;

    let data_ptr = Box::into_raw(Box::new([0u8; 1024])) as *const u8; // Valid pointer to large array
    let length = 1024; // Length of pointer
    let atomic_data = AtomicPtr::new(data_ptr as *mut ());
    
    let vtable = &STATIC_VTABLE; 
    let bytes = unsafe { Bytes::with_vtable(data_ptr, length, atomic_data, vtable) };

    assert_eq!(bytes.ptr, data_ptr); // Validate pointer
    assert_eq!(bytes.len, length); // Validate length
    assert_eq!(bytes.data.load(std::sync::atomic::Ordering::SeqCst), data_ptr as *mut ()); // Validate atomic data
    assert_eq!(bytes.vtable, vtable); // Validate vtable
}

