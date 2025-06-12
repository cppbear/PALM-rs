// Answer 0

#[test]
fn test_promotable_odd_to_vec() {
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};

    struct DummyStruct;

    unsafe fn promotable_to_vec<F>(data: &AtomicPtr<()>, ptr: *const u8, len: usize, f: F) -> Vec<u8>
    where
        F: Fn(*const ()) -> *const u8,
    {
        let slice = std::slice::from_raw_parts(ptr, len);
        let transformed_slice = slice.iter().map(|&byte| f(byte as *const ())).collect::<Vec<_>>();
        transformed_slice.into_iter().map(|p| *p).collect()
    }

    let data = AtomicPtr::new(null());
    let ptr: *const u8 = &[1u8, 2, 3][..] as *const _ as *const u8;
    let len: usize = 3;

    unsafe {
        let result = promotable_odd_to_vec(&data, ptr, len);
        assert_eq!(result, vec![1, 2, 3]);
    }
}

#[test]
#[should_panic]
fn test_promotable_odd_to_vec_with_zero_length() {
    use std::ptr::null;
    use std::sync::atomic::{AtomicPtr, Ordering};

    let data = AtomicPtr::new(null());
    let ptr: *const u8 = &[1u8, 2, 3][..] as *const _ as *const u8;
    let len: usize = 0;

    unsafe {
        let _result = promotable_odd_to_vec(&data, ptr, len);
    }
}

