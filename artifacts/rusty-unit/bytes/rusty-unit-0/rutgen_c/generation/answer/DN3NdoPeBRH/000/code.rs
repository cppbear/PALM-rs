// Answer 0

#[test]
fn test_static_to_mut_valid_pointer() {
    let data = b"Hello, World!";
    let len = data.len();
    let ptr = data.as_ptr();

    // Safety: We ensure the pointer is valid and not dangling
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let bytes_mut = static_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(bytes_mut.len, len);
        assert_eq!(bytes_mut.cap, len); // Assuming cap equals len for this test
    }
}

#[test]
#[should_panic]
fn test_static_to_mut_zero_length() {
    let data = b"";
    let len = data.len();
    let ptr = data.as_ptr();

    // Safety: We ensure the pointer is valid and not dangling
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let bytes_mut = static_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(bytes_mut.len, len);
    }
}

#[test]
fn test_static_to_mut_large_length() {
    let data = b"Hello, World!";
    let len = data.len();
    let ptr = data.as_ptr();

    // Safety: We ensure the pointer is valid and not dangling
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let bytes_mut = static_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(bytes_mut.len, len);
        assert_eq!(bytes_mut.cap, len); // Assuming cap equals len for this test
    }
}

#[test]
fn test_static_to_mut_non_ascii() {
    let data = b"\xC2\xA9"; // Copyright symbol in UTF-8
    let len = data.len();
    let ptr = data.as_ptr();

    // Safety: We ensure the pointer is valid and not dangling
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let bytes_mut = static_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(bytes_mut.len, len);
        assert_eq!(bytes_mut.cap, len); // Assuming cap equals len for this test
    }
}

