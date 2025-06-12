// Answer 0

#[test]
fn test_shallow_clone_vec_success() {
    use core::ptr::null_mut;
    use core::sync::atomic::AtomicPtr;

    let atom = AtomicPtr::new(null_mut());
    let ptr = null_mut();
    let buf: *mut u8 = Box::into_raw(Box::new([0u8; 10])) as *mut u8;
    let offset = buf;
    let len = 10;

    let result = unsafe { shallow_clone_vec(&atom, ptr, buf, offset, len) };

    assert_eq!(result.len, len);
    assert_eq!(result.ptr, offset);
    assert!(!result.data.load(core::sync::atomic::Ordering::Relaxed).is_null());
}

#[test]
fn test_shallow_clone_vec_compare_exchange_failure() {
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicPtr, Ordering};

    let initial_buf: *mut u8 = Box::into_raw(Box::new([0u8; 10])) as *mut u8;
    let atom = AtomicPtr::new(initial_buf as _);
    let ptr = initial_buf as *const ();
    let buf: *mut u8 = Box::into_raw(Box::new([0u8; 10])) as *mut u8;
    let offset = buf;
    let len = 10;

    // Simulate a previous state where a clone has occurred, 
    // so that the compare_exchange fails.
    unsafe {
        atom.store(shallow_clone_vec(&atom, ptr, initial_buf, offset, len).data.load(Ordering::Relaxed), Ordering::Release);
    }

    let result = unsafe { shallow_clone_vec(&atom, ptr, buf, offset, len) };

    assert_eq!(result.len, len);
    assert_eq!(result.ptr, offset);
    assert!(!result.data.load(core::sync::atomic::Ordering::Relaxed).is_null());
}

