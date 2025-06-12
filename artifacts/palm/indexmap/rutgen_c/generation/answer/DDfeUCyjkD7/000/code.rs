// Answer 0

#[test]
fn test_values_debug_fmt() {
    use core::fmt::{self, Write};
    
    struct TestKey;
    struct TestValue;

    let bucket1 = Bucket { hash: 1, key: TestKey, value: TestValue };
    let bucket2 = Bucket { hash: 2, key: TestKey, value: TestValue };
    let buckets = vec![bucket1, bucket2];
    let values = Values { iter: buckets.iter() };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", values);

    assert!(result.is_ok());
    assert!(!output.is_empty());
}

