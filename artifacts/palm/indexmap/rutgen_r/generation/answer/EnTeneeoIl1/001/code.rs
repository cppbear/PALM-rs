// Answer 0

#[derive(Debug, PartialEq)]
struct TryReserveErrorKind {
    kind: String,
}

struct CustomError {
    kind: TryReserveErrorKind,
}

impl CustomError {
    fn from_hashbrown(error: hashbrown::TryReserveError) -> Self {
        Self {
            kind: match error {
                hashbrown::TryReserveError::CapacityOverflow => {
                    TryReserveErrorKind { kind: "CapacityOverflow".to_string() }
                }
                hashbrown::TryReserveError::AllocError { layout } => {
                    TryReserveErrorKind { kind: format!("AllocError with layout: {:?}", layout) }
                }
            },
        }
    }
}

#[test]
fn test_from_hashbrown_alloc_error() {
    use std::alloc::{Layout, GlobalAlloc};
    
    // Creating a layout that might be returned by AllocError
    let layout = Layout::from_size_align(1024, 8).unwrap();

    let alloc_error = hashbrown::TryReserveError::AllocError { layout: layout.clone() };
    let error_instance = CustomError::from_hashbrown(alloc_error);
    
    assert_eq!(error_instance.kind, TryReserveErrorKind { kind: format!("AllocError with layout: {:?}", layout) });
}

#[test]
fn test_from_hashbrown_capacity_overflow() {
    let capacity_overflow_error = hashbrown::TryReserveError::CapacityOverflow;
    let error_instance = CustomError::from_hashbrown(capacity_overflow_error);
    
    assert_eq!(error_instance.kind, TryReserveErrorKind { kind: "CapacityOverflow".to_string() });
}

