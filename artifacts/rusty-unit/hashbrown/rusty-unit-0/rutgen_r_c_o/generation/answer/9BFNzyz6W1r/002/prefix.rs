// Answer 0

#[test]
fn test_to_base_index_with_non_zero_sized_type() {
    use std::ptr::NonNull;

    struct NonZeroSizedType {
        value: i32,
    }

    let layout = std::alloc::Layout::new::<NonZeroSizedType>();
    let base_ptr = unsafe { std::alloc::alloc(layout) };
    let base_non_null = NonNull::new(base_ptr).unwrap();
    
    let bucket = unsafe { Bucket::from_base_index(base_non_null, 1) };

    let index = unsafe { bucket.to_base_index(base_non_null) };

    unsafe { std::alloc::dealloc(base_ptr, layout) };
}

#[test]
fn test_to_base_index_edge_case() {
    use std::ptr::NonNull;

    struct NonZeroSizedType {
        value: i32,
    }

    let layout = std::alloc::Layout::new::<NonZeroSizedType>();
    let base_ptr = unsafe { std::alloc::alloc(layout) };
    let base_non_null = NonNull::new(base_ptr).unwrap();

    let bucket = unsafe { Bucket::from_base_index(base_non_null, 0) };

    let index = unsafe { bucket.to_base_index(base_non_null) };

    unsafe { std::alloc::dealloc(base_ptr, layout) };
}

#[test]
fn test_to_base_index_with_multiple_indices() {
    use std::ptr::NonNull;

    struct NonZeroSizedType {
        value: i32,
    }

    let layout = std::alloc::Layout::new::<NonZeroSizedType>();
    let base_ptr = unsafe { std::alloc::alloc(layout) };
    let base_non_null = NonNull::new(base_ptr).unwrap();
    
    for index in 0..10 {
        let bucket = unsafe { Bucket::from_base_index(base_non_null, index) };
        let result_index = unsafe { bucket.to_base_index(base_non_null) };
    }

    unsafe { std::alloc::dealloc(base_ptr, layout) };
}

