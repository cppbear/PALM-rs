// Answer 0

#[test]
fn test_shared_drop() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ptr;

    let mut atomic_ptr = AtomicPtr::new(ptr::null_mut());

    unsafe {
        shared_drop(&mut atomic_ptr, ptr::null(), 0);
    }

    assert_eq!(atomic_ptr.load(Ordering::SeqCst), ptr::null_mut());
}

#[test]
#[should_panic]
fn test_shared_drop_with_null_pointer() {
    use std::sync::atomic::{AtomicPtr};

    let mut atomic_ptr = AtomicPtr::new(std::ptr::null_mut());

    unsafe {
        shared_drop(&mut atomic_ptr, std::ptr::null(), 0);
    }

    assert_eq!(atomic_ptr.load(Ordering::SeqCst), std::ptr::null_mut());
}

