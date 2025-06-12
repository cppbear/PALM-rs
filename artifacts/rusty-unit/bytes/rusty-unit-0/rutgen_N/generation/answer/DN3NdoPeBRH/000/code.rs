// Answer 0

#[test]
fn test_static_to_mut_valid() {
    use std::ptr::AtomicPtr;
    use bytes::BytesMut;

    let data: [u8; 4] = [1, 2, 3, 4];
    let ptr = data.as_ptr();
    let len = data.len();
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        let result = static_to_mut(&atomic_ptr, ptr, len);
        assert_eq!(result.len(), len);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
        assert_eq!(result[3], 4);
    }
}

#[test]
#[should_panic]
fn test_static_to_mut_invalid_length() {
    use std::ptr::AtomicPtr;
    use bytes::BytesMut;

    let data: [u8; 4] = [1, 2, 3, 4];
    let ptr = data.as_ptr();
    let len = 5; // Invalid length greater than actual data length
    let atomic_ptr = AtomicPtr::new(ptr as *mut ());

    unsafe {
        static_to_mut(&atomic_ptr, ptr, len);
    }
}

