// Answer 0

#[test]
fn test_shared_clone_valid_pointer() {
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use bytes::{Bytes, shallow_clone_arc};

    struct MyBytes {
        data: AtomicPtr<()>,
        ptr: *const u8,
        len: usize,
    }

    let data = AtomicPtr::new(null_mut());
    let ptr = "hello".as_ptr();
    let len = 5;

    let result = unsafe { shared_clone(&data, ptr, len) };
    
    assert_eq!(result.len(), len);
    assert_eq!(result.as_ptr(), ptr);
}

#[test]
#[should_panic]
fn test_shared_clone_null_data() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use bytes::{Bytes, shallow_clone_arc};

    struct MyBytes {
        data: AtomicPtr<()>,
        ptr: *const u8,
        len: usize,
    }

    let data = AtomicPtr::new(null_mut());
    let ptr = null();
    let len = 0;

    let _result = unsafe { shared_clone(&data, ptr, len) };
}

