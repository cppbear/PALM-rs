fn capacity_overflow(self) -> TryReserveError {
        match self {
            Fallibility::Fallible => TryReserveError::CapacityOverflow,
            Fallibility::Infallible => panic!("Hash table capacity overflow"),
        }
    }