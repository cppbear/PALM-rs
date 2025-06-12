#[cfg(any(feature = "std", feature = "alloc"))]
use crate::lib::*;
fn helper(bounds: (usize, Option<usize>)) -> Option<usize> {
    match bounds {
        (lower, Some(upper)) if lower == upper => Some(upper),
        _ => None,
    }
}
