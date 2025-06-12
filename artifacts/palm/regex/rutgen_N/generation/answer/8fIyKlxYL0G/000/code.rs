// Answer 0

#[test]
fn test_is_match_at_valid_offset() {
    struct RegexMatcher;

    impl RegexMatcher {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<()> {
            // Simulating a match for demonstration purposes
            if start < text.len() && text[start] == b'a' {
                Some(())
            } else {
                None
            }
        }
    }

    let matcher = RegexMatcher;
    let text = b"abcde";

    assert!(matcher.is_match_at(text, 0)); // should match at start
    assert!(matcher.is_match_at(text, 1)); // should match at start + 1
    assert!(!matcher.is_match_at(text, 2)); // does not match at start + 2
}

#[test]
fn test_is_match_at_offset_out_of_bounds() {
    struct RegexMatcher;

    impl RegexMatcher {
        fn shortest_match_at(&self, text: &[u8], start: usize) -> Option<()> {
            if start >= text.len() {
                return None; // out of bounds
            }
            // Handling matching logic...
            Some(())
        }
    }

    let matcher = RegexMatcher;
    let text = b"abcde";

    assert!(!matcher.is_match_at(text, 5)); // out of bounds
}

