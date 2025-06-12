// Answer 0

#[derive(Debug)]
struct TryReserveError {
    kind: TryReserveErrorKind,
}

#[derive(Debug)]
enum TryReserveErrorKind {
    CapacityOverflow,
    AllocError { layout: std::alloc::Layout },
}

impl TryReserveError {
    fn from_hashbrown(error: hashbrown::TryReserveError) -> Self {
        Self {
            kind: match error {
                hashbrown::TryReserveError::CapacityOverflow => {
                    TryReserveErrorKind::CapacityOverflow
                }
                hashbrown::TryReserveError::AllocError { layout } => {
                    TryReserveErrorKind::AllocError { layout }
                }
            },
        }
    }
}

#[test]
fn test_from_hashbrown_capacity_overflow() {
    let error = hashbrown::TryReserveError::CapacityOverflow;
    let try_reserve_error = TryReserveError::from_hashbrown(error);
    if let TryReserveErrorKind::CapacityOverflow = try_reserve_error.kind {
        // Test succeeded
    } else {
        panic!("Failed to convert CapacityOverflow error");
    }
}

#[test]
fn test_from_hashbrown_alloc_error() {
    let layout = std::alloc::Layout::from_size_align(1, 1).unwrap();
    let error = hashbrown::TryReserveError::AllocError { layout };
    let try_reserve_error = TryReserveError::from_hashbrown(error);
    if let TryReserveErrorKind::AllocError { layout: l } = try_reserve_error.kind {
        assert_eq!(layout, l);
    } else {
        panic!("Failed to convert AllocError error");
    }
}

