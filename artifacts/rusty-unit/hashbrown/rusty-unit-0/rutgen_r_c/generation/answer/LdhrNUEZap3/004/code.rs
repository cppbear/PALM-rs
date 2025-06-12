// Answer 0

#[test]
fn test_fold_impl_valid_case() {
    use std::ptr::NonNull;
    
    struct TestData {
        value: usize,
    }

    let values = vec![TestData { value: 1 }, TestData { value: 2 }, TestData { value: 3 }, TestData { value: 4 }];
    let layout = std::alloc::Layout::array::<TestData>(values.len()).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) as *mut TestData };
    
    for (i, value) in values.into_iter().enumerate() {
        unsafe {
            ptr.add(i).write(value);
        }
    }

    let bucket = unsafe { Bucket { ptr: NonNull::new(ptr).unwrap() } };
    let ctrl = ptr as *const u8;

    let iter_range = unsafe { RawIterRange::new(ctrl, bucket, values.len()) };

    let result = unsafe {
        iter_range.fold_impl(values.len(), 0, |acc, bucket| {
            acc + bucket.as_ref().value
        })
    };

    assert_eq!(result, 10);

    unsafe { std::alloc::dealloc(ptr as *mut u8, layout) };
}

#[test]
#[should_panic]
fn test_fold_impl_panic_due_to_empty_n() {
    struct TestData {
        value: usize,
    }

    let values = vec![TestData { value: 1 }];
    let layout = std::alloc::Layout::array::<TestData>(values.len()).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) as *mut TestData };

    for (i, value) in values.into_iter().enumerate() {
        unsafe {
            ptr.add(i).write(value);
        }
    }

    let bucket = unsafe { Bucket { ptr: NonNull::new(ptr).unwrap() } };
    let ctrl = ptr as *const u8;

    let iter_range = unsafe { RawIterRange::new(ctrl, bucket, values.len()) };

    // This should panic since n is not zero
    unsafe {
        iter_range.fold_impl(0, 0, |acc, _| acc);
    }

    unsafe { std::alloc::dealloc(ptr as *mut u8, layout) };
}

