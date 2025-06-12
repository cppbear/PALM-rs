// Answer 0

#[test]
fn test_third_with_different_types() {
    let tuple = (1, "test", 3.14);
    assert_eq!(third(tuple), 3.14);
}

#[test]
fn test_third_with_same_types() {
    let tuple = (5, 10, 15);
    assert_eq!(third(tuple), 15);
}

#[test]
fn test_third_with_empty_string() {
    let tuple = (0, "", "last");
    assert_eq!(third(tuple), "last");
}

#[test]
fn test_third_with_complex_types() {
    struct TestStruct;
    let tuple = (TestStruct, TestStruct, TestStruct);
    let result: TestStruct = third(tuple);
    // Can add further assertions if needed for struct comparison
}

