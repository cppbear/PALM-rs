use core::ops::{Bound, Range, RangeBounds};
pub(crate) fn third<A, B, C>(t: (A, B, C)) -> C {
    t.2
}
