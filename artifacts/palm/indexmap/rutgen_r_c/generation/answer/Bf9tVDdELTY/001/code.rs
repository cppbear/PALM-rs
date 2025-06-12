// Answer 0

#[test]
fn test_ref_mut() {
    #[derive(Debug, PartialEq)]
    struct TestKey(i32);
    #[derive(Debug, PartialEq)]
    struct TestValue(i32);

    let mut bucket = Bucket {
        hash: HashValue(1),
        key: TestKey(10),
        value: TestValue(20),
    };

    let (key_ref, value_mut) = bucket.ref_mut();
    
    // Verify the key reference and mutable value reference
    assert_eq!(*key_ref, TestKey(10));
    value_mut.0 += 5;  // Modify the value through the mutable reference

    // Verify that the modification is reflected in the bucket
    assert_eq!(bucket.value, TestValue(25));
}

