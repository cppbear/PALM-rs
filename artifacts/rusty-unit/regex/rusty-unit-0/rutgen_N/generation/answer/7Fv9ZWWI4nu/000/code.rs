// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMatch<'t> {
        text: &'t str,
        start: usize,
        end: usize,
    }

    impl<'t> TestMatch<'t> {
        fn as_str(&self) -> &'t str {
            &self.text[self.start..self.end]
        }
    }

    #[test]
    fn test_as_str_valid_range() {
        let text = "Hello, world!";
        let m = TestMatch { text, start: 7, end: 12 };
        assert_eq!(m.as_str(), "world");
    }

    #[test]
    fn test_as_str_empty_match() {
        let text = "Hello";
        let m = TestMatch { text, start: 5, end: 5 };
        assert_eq!(m.as_str(), "");
    }

    #[test]
    #[should_panic]
    fn test_as_str_out_of_bounds_start() {
        let text = "Hello, world!";
        let m = TestMatch { text, start: 13, end: 15 };
        let _ = m.as_str(); // This should panic
    }

    #[test]
    #[should_panic]
    fn test_as_str_out_of_bounds_end() {
        let text = "Hello, world!";
        let m = TestMatch { text, start: 7, end: 20 };
        let _ = m.as_str(); // This should panic
    }

    #[test]
    #[should_panic]
    fn test_as_str_invalid_range() {
        let text = "Hello, world!";
        let m = TestMatch { text, start: 10, end: 5 };
        let _ = m.as_str(); // This should panic due to invalid range
    }
}

