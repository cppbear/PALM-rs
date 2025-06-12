// Answer 0

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_capacity_overflow_infallible() {
    struct TestStruct;

    impl TestStruct {
        fn capacity_overflow(self) -> TryReserveError {
            Fallibility::Infallible.capacity_overflow()
        }
    }

    let my_struct = TestStruct;
    let _ = my_struct.capacity_overflow();
}

#[test]
fn test_capacity_overflow_fallible() {
    let result = Fallibility::Fallible.capacity_overflow();
    match result {
        TryReserveError::CapacityOverflow => assert!(true),
        _ => assert!(false, "Expected CapacityOverflow error"),
    }
}

