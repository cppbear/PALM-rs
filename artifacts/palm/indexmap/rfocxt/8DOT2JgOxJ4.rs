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
impl core::fmt::Display for TryReserveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let reason = match &self.kind {
            TryReserveErrorKind::Std(e) => return core::fmt::Display::fmt(e, f),
            TryReserveErrorKind::CapacityOverflow => {
                " because the computed capacity exceeded the collection's maximum"
            }
            TryReserveErrorKind::AllocError { .. } => {
                " because the memory allocator returned an error"
            }
        };
        f.write_str("memory allocation failed")?;
        f.write_str(reason)
    }
}
