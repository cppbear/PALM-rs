// Answer 0

#[test]
fn test_promotable_even_to_mut_valid() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use bytes::BytesMut;
    use std::ptr;

    let data: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());
    let ptr: *const u8 = &42u8 as *const u8;
    let len: usize = 1;

    unsafe {
        let result = promotable_even_to_mut(&data, ptr, len);
        assert_eq!(result.len(), len);
    }
}

#[test]
#[should_panic]
fn test_promotable_even_to_mut_invalid_length() {
    use std::sync::atomic::{AtomicPtr, Ordering};
    use bytes::BytesMut;
    use std::ptr;

    let data: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());
    let ptr: *const u8 = &42u8 as *const u8;
    let len: usize = 0;

    unsafe {
        let _result = promotable_even_to_mut(&data, ptr, len);
    }
}

