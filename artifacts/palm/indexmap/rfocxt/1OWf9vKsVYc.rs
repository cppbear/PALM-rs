use alloc::vec::{self, Vec};
pub use crate::map::IndexMap;
pub use crate::set::IndexSet;
pub use equivalent::Equivalent;
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TryReserveError {
    kind: TryReserveErrorKind,
}
#[derive(Clone, PartialEq, Eq, Debug)]
enum TryReserveErrorKind {
    Std(alloc::collections::TryReserveError),
    CapacityOverflow,
    AllocError { layout: alloc::alloc::Layout },
}
impl TryReserveError {
    fn from_alloc(error: alloc::collections::TryReserveError) -> Self {
        Self {
            kind: TryReserveErrorKind::Std(error),
        }
    }
    fn from_hashbrown(error: hashbrown::TryReserveError) -> Self {
        Self {
            kind: match error {
                hashbrown::TryReserveError::CapacityOverflow => {
                    TryReserveErrorKind::CapacityOverflow
                }
                hashbrown::TryReserveError::AllocError { layout } => {
                    TryReserveErrorKind::AllocError {
                        layout,
                    }
                }
            },
        }
    }
}
