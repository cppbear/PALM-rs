// Answer 0

#[test]
fn test_is_match_at_start_zero() {
    struct TestRegex {
        pattern: &'static str,
    }

    impl TestRegex {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
            if start == 0 && text.starts_with(self.pattern) {
                Some(0)
            } else {
                None
            }
        }
    }

    let regex = TestRegex { pattern: "abc" };
    assert_eq!(regex.is_match_at("abcde", 0), true);
}

#[test]
fn test_is_match_at_start_non_zero() {
    struct TestRegex {
        pattern: &'static str,
    }

    impl TestRegex {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
            if start > 0 && text[start..].starts_with(self.pattern) {
                Some(start)
            } else {
                None
            }
        }
    }

    let regex = TestRegex { pattern: "abc" };
    assert_eq!(regex.is_match_at("abcde", 1), false);
}

#[test]
fn test_is_match_at_empty_string() {
    struct TestRegex {
        pattern: &'static str,
    }

    impl TestRegex {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
            if start == 0 && text.is_empty() {
                None
            } else if text[start..].starts_with(self.pattern) {
                Some(start)
            } else {
                None
            }
        }
    }

    let regex = TestRegex { pattern: "abc" };
    assert_eq!(regex.is_match_at("", 0), false);
}

#[test]
fn test_is_match_at_partial_match() {
    struct TestRegex {
        pattern: &'static str,
    }

    impl TestRegex {
        fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
            if start < text.len() && text[start..].starts_with(self.pattern) {
                Some(start)
            } else {
                None
            }
        }
    }

    let regex = TestRegex { pattern: "abc" };
    assert_eq!(regex.is_match_at("deabcde", 2), true);
}

