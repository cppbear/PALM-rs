// Answer 0

#[test]
fn test_capacity_overflow_fallible() {
    let fallibility = Fallibility::Fallible;
    let result = fallibility.capacity_overflow();
    assert_eq!(result, TryReserveError::CapacityOverflow);
}

#[should_panic(expected = "Hash table capacity overflow")]
#[test]
fn test_capacity_overflow_infallible() {
    let fallibility = Fallibility::Infallible;
    fallibility.capacity_overflow();
}

