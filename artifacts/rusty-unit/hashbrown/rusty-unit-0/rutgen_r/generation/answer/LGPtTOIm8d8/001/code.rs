// Answer 0

#[test]
#[should_panic(expected = "Hash table capacity overflow")]
fn test_capacity_overflow_infallible() {
    struct Fallibility {
        infallible: bool,
    }

    impl Fallibility {
        fn capacity_overflow(self) -> TryReserveError {
            match self {
                Fallibility::Fallible => TryReserveError::CapacityOverflow,
                Fallibility::Infallible => panic!("Hash table capacity overflow"),
            }
        }
    }

    let infallible_instance = Fallibility { infallible: true };
    infallible_instance.capacity_overflow();
}

