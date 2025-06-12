// Answer 0

#[derive(Debug)]
enum Fallibility {
    Fallible,
    Infallible,
}

#[derive(Debug)]
struct TryReserveError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    CapacityOverflow,
}

impl Fallibility {
    fn capacity_overflow(self) -> TryReserveError {
        match self {
            Fallibility::Fallible => TryReserveError { kind: ErrorKind::CapacityOverflow },
            Fallibility::Infallible => panic!("Hash table capacity overflow"),
        }
    }
}

#[test]
fn test_capacity_overflow_fallible() {
    let fallible = Fallibility::Fallible;
    let error = fallible.capacity_overflow();
    assert_eq!(error.kind, ErrorKind::CapacityOverflow);
}

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_capacity_overflow_infallible() {
    let infallible = Fallibility::Infallible;
    infallible.capacity_overflow();
}

