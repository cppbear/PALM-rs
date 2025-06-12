use core::ops::{Bound, Range, RangeBounds};
pub(crate) fn slice_eq<T, U>(
    left: &[T],
    right: &[U],
    eq: impl Fn(&T, &U) -> bool,
) -> bool {
    if left.len() != right.len() {
        return false;
    }
    for i in 0..left.len() {
        if !eq(&left[i], &right[i]) {
            return false;
        }
    }
    true
}
