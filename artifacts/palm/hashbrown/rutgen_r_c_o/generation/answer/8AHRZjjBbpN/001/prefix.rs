// Answer 0

#[test]
fn test_as_ptr_zero_sized() {
    struct ZeroSized;

    impl ZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let bucket = Bucket {
        ptr: NonNull::new(0 as *mut ZeroSized).unwrap(),
    };
    
    let _result = bucket.as_ptr();
}

#[test]
fn test_as_ptr_zero_sized_multiple_instances() {
    struct AnotherZeroSized;

    impl AnotherZeroSized {
        const IS_ZERO_SIZED: bool = true;
    }

    let bucket1 = Bucket {
        ptr: NonNull::new(0 as *mut AnotherZeroSized).unwrap(),
    };

    let _result1 = bucket1.as_ptr();

    let bucket2 = Bucket {
        ptr: NonNull::new(0 as *mut AnotherZeroSized).unwrap(),
    };

    let _result2 = bucket2.as_ptr();
}

