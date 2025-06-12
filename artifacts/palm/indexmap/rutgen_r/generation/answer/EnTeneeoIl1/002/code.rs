// Answer 0

#[derive(Debug)]
enum TryReserveErrorKind {
    CapacityOverflow,
    AllocError { layout: std::alloc::Layout },
}

#[derive(Debug)]
struct MyStruct {
    kind: TryReserveErrorKind,
}

impl MyStruct {
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
    let result = MyStruct::from_hashbrown(error);
    assert!(matches!(result.kind, TryReserveErrorKind::CapacityOverflow));
}

#[test]
fn test_from_hashbrown_alloc_error() {
    let layout = std::alloc::Layout::from_size_align(1, 1).unwrap();
    let error = hashbrown::TryReserveError::AllocError { layout };
    let result = MyStruct::from_hashbrown(error);
    assert!(matches!(result.kind, TryReserveErrorKind::AllocError { .. }));
}

