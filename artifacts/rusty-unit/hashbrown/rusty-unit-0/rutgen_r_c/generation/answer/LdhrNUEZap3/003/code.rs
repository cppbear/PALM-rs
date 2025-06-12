// Answer 0

#[test]
fn test_fold_impl_with_multiple_iterations() {
    struct TestBucket;

    impl TestBucket {
        const IS_ZERO_SIZED: bool = false; // Adjust based on your scenario
    }

    let base = NonNull::new(Box::into_raw(Box::new(TestBucket))).unwrap();
    let bucket = Bucket {
        ptr: base,
    };

    let iter_range = RawIterRange::new(std::ptr::null(), bucket, 4); // Initialize with 4 buckets

    unsafe {
        let result: usize = iter_range.fold_impl(4, 0, |acc, _| acc + 1); // Expecting result to be 4
        assert_eq!(result, 4);
    }
}

#[test]
fn test_fold_impl_with_no_elements() {
    struct TestBucket;

    impl TestBucket {
        const IS_ZERO_SIZED: bool = false; 
    }

    let base = NonNull::new(Box::into_raw(Box::new(TestBucket))).unwrap();
    let bucket = Bucket {
        ptr: base,
    };

    let iter_range = RawIterRange::new(std::ptr::null(), bucket, 0); // Initialize with zero buckets

    unsafe {
        let result: usize = iter_range.fold_impl(0, 10, |acc, _| acc + 1); // Should return initial value
        assert_eq!(result, 10);
    }
}

#[test]
#[should_panic]
fn test_fold_impl_with_invalid_n() {
    struct TestBucket;

    impl TestBucket {
        const IS_ZERO_SIZED: bool = false; 
    }

    let base = NonNull::new(Box::into_raw(Box::new(TestBucket))).unwrap();
    let bucket = Bucket {
        ptr: base,
    };

    let iter_range = RawIterRange::new(std::ptr::null(), bucket, 1); // Initialize with 1 bucket

    unsafe {
        let _result: usize = iter_range.fold_impl(0, 0, |acc, _| acc + 1); // Should panic, n must not be 0
    }
} 

#[test]
fn test_fold_impl_with_single_iteration() {
    struct TestBucket;

    impl TestBucket {
        const IS_ZERO_SIZED: bool = false; 
    }

    let base = NonNull::new(Box::into_raw(Box::new(TestBucket))).unwrap();
    let bucket = Bucket {
        ptr: base,
    };

    let iter_range = RawIterRange::new(std::ptr::null(), bucket, 1); // Initialize with 1 bucket
    
    unsafe {
        let result: usize = iter_range.fold_impl(1, 0, |acc, _| acc + 1); // Expecting result to be 1
        assert_eq!(result, 1);
    }
} 

#[test]
fn test_fold_impl_with_no_buckets() {
    struct TestBucket;

    impl TestBucket {
        const IS_ZERO_SIZED: bool = false; 
    }

    let base = NonNull::new(Box::into_raw(Box::new(TestBucket))).unwrap();
    let bucket = Bucket {
        ptr: base,
    };

    let iter_range = RawIterRange::new(std::ptr::null(), bucket, 0); // No buckets present

    unsafe {
        let result: usize = iter_range.fold_impl(0, 5, |acc, _| acc + 1); // Initial value should return
        assert_eq!(result, 5);
    }
}

