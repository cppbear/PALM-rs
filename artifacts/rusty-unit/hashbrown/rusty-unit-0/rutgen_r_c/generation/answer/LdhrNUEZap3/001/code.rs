// Answer 0

#[test]
fn test_fold_impl_non_empty() {
    struct TestBucket {
        value: i32,
    }

    let group_width = 4; // Example group width; replace with actual value
    let mut bitmask = BitMask(0b1111); // All group indices populated
    let non_null_ptr = NonNull::new(Box::into_raw(Box::new(TestBucket { value: 0 }))).unwrap();

    let bucket = Bucket { ptr: non_null_ptr };

    let raw_iter_range = RawIterRange {
        current_group: bitmask.into_iter(),
        data: bucket,
        next_ctrl: ptr::null(),
        end: ptr::null(),
    };

    unsafe {
        let result = raw_iter_range.fold_impl(4, 0, |acc, bucket| acc + bucket.as_ref().value);
        assert_eq!(result, 0); // Change the expected value based on your logic
    }
}

#[test]
fn test_fold_impl_empty() {
    struct TestBucket {
        value: i32,
    }

    let mut bitmask = BitMask(0b0000); // No group indices populated
    let non_null_ptr = NonNull::new(Box::into_raw(Box::new(TestBucket { value: 0 }))).unwrap();

    let bucket = Bucket { ptr: non_null_ptr };

    let raw_iter_range = RawIterRange {
        current_group: bitmask.into_iter(),
        data: bucket,
        next_ctrl: ptr::null(),
        end: ptr::null(),
    };

    unsafe {
        let result = raw_iter_range.fold_impl(0, 5, |acc, bucket| acc + bucket.as_ref().value);
        assert_eq!(result, 5); // Initial accumulator should be returned
    }
}

