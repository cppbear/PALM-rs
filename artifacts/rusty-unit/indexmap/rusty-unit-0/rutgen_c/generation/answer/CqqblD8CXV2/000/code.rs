// Answer 0

#[test]
fn test_slice_debug_fmt() {
    struct TestKey(i32);
    struct TestValue(i32);
    
    let entries = [
        Bucket { hash: 0, key: TestKey(1), value: TestValue(10) },
        Bucket { hash: 0, key: TestKey(2), value: TestValue(20) },
        Bucket { hash: 0, key: TestKey(3), value: TestValue(30) },
    ];
    
    let slice = Slice { entries };

    let result = format!("{:?}", slice);
    assert!(result.contains("TestKey(1)"));
    assert!(result.contains("TestValue(10)"));
    assert!(result.contains("TestKey(2)"));
    assert!(result.contains("TestValue(20)"));
    assert!(result.contains("TestKey(3)"));
    assert!(result.contains("TestValue(30)"));
}

