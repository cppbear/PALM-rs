// Answer 0

#[test]
fn test_from_base_index_non_zero_sized() {
    use std::ptr::NonNull;

    struct Dummy {
        value: u32,
    }

    let dummy_array = vec![Dummy { value: 1 }, Dummy { value: 2 }];
    let base = NonNull::new(dummy_array.as_ptr() as *mut Dummy).unwrap();
    let index = 1;

    let bucket = unsafe { from_base_index(base, index) };
    assert_eq!(bucket.ptr.as_ref().value, 2);
}

#[test]
fn test_from_base_index_zero_sized() {
    use std::ptr::NonNull;

    struct ZST;

    let zst_array = vec![ZST, ZST];
    let base = NonNull::new(zst_array.as_ptr() as *mut ZST).unwrap();
    let index = 0;

    let bucket = unsafe { from_base_index(base, index) };
    // For Zero-Sized Types, we can't assert on value but we can check that the pointer is valid.
    assert!(!bucket.ptr.as_ptr().is_null());
}

#[test]
#[should_panic]
fn test_from_base_index_out_of_bounds() {
    use std::ptr::NonNull;

    struct Dummy {
        value: u32,
    }

    let dummy_array = vec![Dummy { value: 1 }];
    let base = NonNull::new(dummy_array.as_ptr() as *mut Dummy).unwrap();
    let index = 2; // Out of bounds index

    unsafe { from_base_index(base, index) }; // This should panic
}

