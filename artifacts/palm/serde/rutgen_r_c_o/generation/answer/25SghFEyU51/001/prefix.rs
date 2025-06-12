// Answer 0

#[test]
fn test_from_bounds_eq_zero() {
    let vec: Vec<i32> = Vec::new();
    let iter = vec.iter();
    from_bounds(&iter);
}

#[test]
fn test_from_bounds_eq_one() {
    let vec = vec![1];
    let iter = vec.iter();
    from_bounds(&iter);
}

#[test]
fn test_from_bounds_eq_two() {
    let vec = vec![1, 2];
    let iter = vec.iter();
    from_bounds(&iter);
}

#[test]
fn test_from_bounds_lower_greater_than_upper() {
    struct LowerGreaterThanUpper;

    impl Iterator for LowerGreaterThanUpper {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (2, Some(1))
        }
    }

    let iter = LowerGreaterThanUpper;
    from_bounds(&iter);
}

#[test]
fn test_from_bounds_upper_none() {
    struct UpperNone;

    impl Iterator for UpperNone {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, None)
        }
    }

    let iter = UpperNone;
    from_bounds(&iter);
}

