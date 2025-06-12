// Answer 0

#[test]
fn test_set_when_full() {
    struct TestData;

    let cell = OnceRef::<TestData>::new();
    let value = TestData;

    // First, we need to set the value to something to simulate a full state
    let _ = cell.set(&value); // This should succeed

    // Now, we try to set it again, expecting an Err
    let result = cell.set(&value); 
    assert_eq!(result, Err(()));
}

