// Answer 0

#[test]
fn test_static_clone_valid() {
    let data: &[u8] = b"Test data";
    let ptr = data.as_ptr();
    let len = data.len();
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let result = static_clone(&atomic_ptr, ptr, len);
        assert_eq!(result.len(), len);
        assert_eq!(unsafe { core::slice::from_raw_parts(result.ptr, result.len) }, data);
    }
}

#[should_panic(expected = "out of range")]
#[test]
fn test_static_clone_zero_length() {
    let data: &[u8] = b"Test data";
    let ptr = data.as_ptr();
    let len = 0;
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        static_clone(&atomic_ptr, ptr, len);
    }
}

#[should_panic(expected = "out of bounds")]
#[test]
fn test_static_clone_out_of_bounds() {
    let data: &[u8] = b"Test data";
    let ptr = data.as_ptr();
    let len = data.len() + 1; // setting length to an out of bounds value
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        static_clone(&atomic_ptr, ptr, len);
    }
}

