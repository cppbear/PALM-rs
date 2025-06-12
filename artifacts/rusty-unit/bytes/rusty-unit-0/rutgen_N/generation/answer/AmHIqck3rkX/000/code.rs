// Answer 0

#[cfg(test)]
use std::ptr::AtomicPtr;
use bytes::BytesMut;

unsafe fn owned_to_vec(data: &AtomicPtr<()>, ptr: *const u8, len: usize) -> Vec<u8> {
    // Implementation for the sake of testing
    ptr::slice_from_raw_parts(ptr, len as usize).to_vec()
}

#[test]
fn test_owned_to_mut_with_valid_pointer() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let input_data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let ptr: *const u8 = input_data.as_ptr();
    let len: usize = input_data.len();

    unsafe {
        let result = owned_to_mut(&data, ptr, len);
        assert_eq!(result.as_ref(), &input_data[..]);
    }
}

#[test]
#[should_panic]
fn test_owned_to_mut_with_null_pointer() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let ptr: *const u8 = std::ptr::null();
    let len: usize = 5;

    unsafe {
        owned_to_mut(&data, ptr, len);
    }
}

#[test]
fn test_owned_to_mut_with_zero_length() {
    let data = AtomicPtr::new(std::ptr::null_mut());
    let ptr: *const u8 = std::ptr::null();
    let len: usize = 0;

    unsafe {
        let result = owned_to_mut(&data, ptr, len);
        assert!(result.is_empty());
    }
}

