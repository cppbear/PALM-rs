use core::ops::{Bound, Range, RangeBounds};

pub(crate) fn third<A, B, C>(t: (A, B, C)) -> C {
    t.2
}

#[track_caller]
pub(crate) fn simplify_range<R>(range: R, len: usize) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&i) if i <= len => i,
        Bound::Excluded(&i) if i < len => i + 1,
        Bound::Included(i) | Bound::Excluded(i) => {
            panic!("range start index {i} out of range for slice of length {len}")
        }
    };
    let end = match range.end_bound() {
        Bound::Unbounded => len,
        Bound::Excluded(&i) if i <= len => i,
        Bound::Included(&i) if i < len => i + 1,
        Bound::Included(i) | Bound::Excluded(i) => {
            panic!("range end index {i} out of range for slice of length {len}")
        }
    };
    if start > end {
        panic!(
            "range start index {:?} should be <= range end index {:?}",
            range.start_bound(),
            range.end_bound()
        );
    }
    start..end
}

pub(crate) fn try_simplify_range<R>(range: R, len: usize) -> Option<Range<usize>>
where
    R: RangeBounds<usize>,
{
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&i) if i <= len => i,
        Bound::Excluded(&i) if i < len => i + 1,
        _ => return None,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => len,
        Bound::Excluded(&i) if i <= len => i,
        Bound::Included(&i) if i < len => i + 1,
        _ => return None,
    };
    if start > end {
        return None;
    }
    Some(start..end)
}

// Generic slice equality -- copied from the standard library but adding a custom comparator,
// allowing for our `Bucket` wrapper on either or both sides.
pub(crate) fn slice_eq<T, U>(left: &[T], right: &[U], eq: impl Fn(&T, &U) -> bool) -> bool {
    if left.len() != right.len() {
        return false;
    }

    // Implemented as explicit indexing rather
    // than zipped iterators for performance reasons.
    // See PR https://github.com/rust-lang/rust/pull/116846
    for i in 0..left.len() {
        // bound checks are optimized away
        if !eq(&left[i], &right[i]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests_llm_16_724 {
    use crate::util::slice_eq;

    #[test]
    fn test_slice_eq_equal() {
        let left = [1, 2, 3];
        let right = [1, 2, 3];
        let eq = |a: &i32, b: &i32| a == b;

        assert!(slice_eq(&left, &right, eq));
    }

    #[test]
    fn test_slice_eq_unequal_length() {
        let left = [1, 2, 3];
        let right = [1, 2];
        let eq = |a: &i32, b: &i32| a == b;

        assert!(!slice_eq(&left, &right, eq));
    }

    #[test]
    fn test_slice_eq_unequal_elements() {
        let left = [1, 2, 3];
        let right = [1, 2, 4];
        let eq = |a: &i32, b: &i32| a == b;

        assert!(!slice_eq(&left, &right, eq));
    }

    #[test]
    fn test_slice_eq_with_strings() {
        let left = ["hello", "world"];
        let right = ["hello", "world"];
        let eq = |a: &&str, b: &&str| a == b;

        assert!(slice_eq(&left, &right, eq));
    }

    #[test]
    fn test_slice_eq_with_different_string_order() {
        let left = ["hello", "world"];
        let right = ["world", "hello"];
        let eq = |a: &&str, b: &&str| a == b;

        assert!(!slice_eq(&left, &right, eq));
    }
}

#[cfg(test)]
mod tests_llm_16_725 {
    use super::*;

use crate::*;

    #[test]
    fn test_third() {
        assert_eq!(third((1, 2, 3)), 3);
        assert_eq!(third(("a", "b", "c")), "c");
        assert_eq!(third((1.0, 2.0, 3.0)), 3.0);
        assert_eq!(third((true, false, true)), true);
    }
}
