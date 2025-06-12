// Answer 0

#[test]
fn test_with_vtable_valid() {
    use core::ptr::NonNull;
    use alloc::alloc::AllocErr;

    let data: AtomicPtr<()> = AtomicPtr::new(NonNull::dangling().as_ptr() as *mut ());
    let bytes = unsafe {
        Bytes::with_vtable(
            NonNull::dangling().as_ptr(),
            5,
            data,
            &STATIC_VTABLE,
        )
    };

    assert_eq!(bytes.len, 5);
    assert_eq!(bytes.ptr, NonNull::dangling().as_ptr());
    assert!(bytes.data.load(Ordering::SeqCst).is_null());
    assert_eq!(bytes.vtable, &STATIC_VTABLE);
}

#[test]
fn test_with_vtable_zero_length() {
    let data: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
    let bytes = unsafe {
        Bytes::with_vtable(
            NonNull::dangling().as_ptr(),
            0,
            data,
            &STATIC_VTABLE,
        )
    };

    assert_eq!(bytes.len, 0);
    assert_eq!(bytes.ptr, NonNull::dangling().as_ptr());
    assert!(bytes.data.load(Ordering::SeqCst).is_null());
    assert_eq!(bytes.vtable, &STATIC_VTABLE);
}

#[should_panic]
fn test_with_vtable_null_pointer() {
    let data: AtomicPtr<()> = AtomicPtr::new(core::ptr::null_mut());
    unsafe {
        Bytes::with_vtable(
            core::ptr::null(),
            5,
            data,
            &STATIC_VTABLE,
        );
    }
}

