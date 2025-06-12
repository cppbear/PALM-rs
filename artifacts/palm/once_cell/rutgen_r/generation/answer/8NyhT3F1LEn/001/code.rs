// Answer 0

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    fn to_usize(value: bool) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(if value { 1 } else { 2 }) }
    }

    #[test]
    fn test_to_usize_true() {
        let result = to_usize(true);
        assert_eq!(result, NonZeroUsize::new(1).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_to_usize_false() {
        let _result = to_usize(false);
    }
}

