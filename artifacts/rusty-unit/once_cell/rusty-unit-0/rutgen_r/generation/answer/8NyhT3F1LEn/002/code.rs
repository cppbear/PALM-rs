// Answer 0

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    fn to_usize(value: bool) -> NonZeroUsize {
        unsafe { NonZeroUsize::new_unchecked(if value { 1 } else { 2 }) }
    }

    #[test]
    fn test_to_usize_with_false() {
        let result = to_usize(false);
        assert_eq!(result.get(), 2);
    }

    #[should_panic]
    fn test_to_usize_with_true() {
        let _ = to_usize(true);
    }
}

