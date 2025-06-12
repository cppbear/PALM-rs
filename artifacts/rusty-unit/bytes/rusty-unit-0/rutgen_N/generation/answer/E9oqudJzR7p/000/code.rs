// Answer 0

#[cfg(test)]
mod tests {
    const MIN_ORIGINAL_CAPACITY_WIDTH: usize = 3;

    fn original_capacity_from_repr(repr: usize) -> usize {
        if repr == 0 {
            return 0;
        }

        1 << (repr + (MIN_ORIGINAL_CAPACITY_WIDTH - 1))
    }

    #[test]
    fn test_original_capacity_from_repr_zero() {
        assert_eq!(original_capacity_from_repr(0), 0);
    }

    #[test]
    fn test_original_capacity_from_repr_one() {
        assert_eq!(original_capacity_from_repr(1), 8);
    }

    #[test]
    fn test_original_capacity_from_repr_two() {
        assert_eq!(original_capacity_from_repr(2), 16);
    }

    #[test]
    fn test_original_capacity_from_repr_three() {
        assert_eq!(original_capacity_from_repr(3), 32);
    }

    #[test]
    fn test_original_capacity_from_repr_large() {
        assert_eq!(original_capacity_from_repr(10), 8192);
    }
}

