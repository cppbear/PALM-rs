// Answer 0

#[test]
fn test_static_to_mut_valid_pointer() {
    use std::ptr;

    let data = b"Hello, world!";
    let len = data.len();
    let ptr = data.as_ptr();

    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let bytes_mut = static_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(bytes_mut.len, len);
        assert_eq!(bytes_mut.ptr.as_ptr(), ptr as *mut u8);
    }
}

#[test]
fn test_static_to_mut_zero_length() {
    use std::ptr;

    let data: &[u8] = &[];
    let len = data.len();
    let ptr = data.as_ptr();

    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let bytes_mut = static_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(bytes_mut.len, len);
        assert_eq!(bytes_mut.ptr.as_ptr(), ptr as *mut u8);
    }
}

#[should_panic]
fn test_static_to_mut_null_pointer() {
    let atomic_ptr = AtomicPtr::new(ptr::null_mut());

    unsafe {
        let _bytes_mut = static_to_mut(&atomic_ptr, ptr::null(), 10);
    }
}

#[should_panic]
fn test_static_to_mut_invalid_length() {
    use std::ptr;

    let data = b"Hello, world!";
    let len = 20; // Length exceeds actual length
    let ptr = data.as_ptr();

    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let _bytes_mut = static_to_mut(&atomic_ptr, ptr, len);
    }
}

