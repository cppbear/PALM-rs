// Answer 0

#[derive(Debug)]
enum Fallibility {
    Fallible,
    Infallible,
}

#[derive(Debug, PartialEq)]
enum TryReserveError {
    CapacityOverflow,
}

impl Fallibility {
    fn capacity_overflow(self) -> TryReserveError {
        match self {
            Fallibility::Fallible => TryReserveError::CapacityOverflow,
            Fallibility::Infallible => panic!("Hash table capacity overflow"),
        }
    }
}

#[test]
fn test_capacity_overflow_fallible() {
    let fallible = Fallibility::Fallible;
    let result = fallible.capacity_overflow();
    assert_eq!(result, TryReserveError::CapacityOverflow);
}

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_capacity_overflow_infallible() {
    let infallible = Fallibility::Infallible;
    infallible.capacity_overflow();
}

