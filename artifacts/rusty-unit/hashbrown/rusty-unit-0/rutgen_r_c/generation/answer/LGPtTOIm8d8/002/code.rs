// Answer 0

#[test]
fn test_capacity_overflow_fallible() {
    let fallibility = Fallibility::Fallible;
    let result = fallibility.capacity_overflow();
    match result {
        TryReserveError::CapacityOverflow => {},
        _ => panic!("Expected CapacityOverflow, got {:?}", result),
    }
}

#[should_panic(expected = "Hash table capacity overflow")]
#[test]
fn test_capacity_overflow_infallible() {
    let fallibility = Fallibility::Infallible;
    fallibility.capacity_overflow();
}

