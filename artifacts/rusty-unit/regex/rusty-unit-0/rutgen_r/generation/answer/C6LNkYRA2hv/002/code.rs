// Answer 0

#[test]
fn test_is_suffix_equal_length() {
    struct Pattern {
        pat: Vec<u8>,
    }

    impl Pattern {
        fn len(&self) -> usize {
            self.pat.len()
        }
    }

    let pattern = Pattern { pat: b"test".to_vec() }; // Pattern with 4 bytes
    let text = b"test"; // Text with exactly the same length as the pattern
    assert!(pattern.is_suffix(text));
}

#[test]
fn test_is_suffix_not_equal() {
    struct Pattern {
        pat: Vec<u8>,
    }

    impl Pattern {
        fn len(&self) -> usize {
            self.pat.len()
        }
    }

    let pattern = Pattern { pat: b"test".to_vec() }; // Pattern with 4 bytes
    let text = b"jest"; // Text with exactly the same length but different content
    assert!(!pattern.is_suffix(text));
}

#[test]
fn test_is_suffix_empty_pattern() {
    struct Pattern {
        pat: Vec<u8>,
    }

    impl Pattern {
        fn len(&self) -> usize {
            self.pat.len()
        }
    }

    let pattern = Pattern { pat: b"".to_vec() }; // Empty pattern
    let text = b""; // Empty text, should be a suffix of an empty string
    assert!(pattern.is_suffix(text));
}

#[test]
fn test_is_suffix_text_longer_than_pattern() {
    struct Pattern {
        pat: Vec<u8>,
    }

    impl Pattern {
        fn len(&self) -> usize {
            self.pat.len()
        }
    }

    let pattern = Pattern { pat: b"test".to_vec() }; // Pattern with 4 bytes
    let text = b"this is a test"; // Text is longer than the pattern
    assert!(pattern.is_suffix(&text[11..])); // Last 4 bytes "test"
}

#[should_panic]
#[test]
fn test_is_suffix_panic_condition() {
    struct Pattern {
        pat: Vec<u8>,
    }

    impl Pattern {
        fn len(&self) -> usize {
            self.pat.len()
        }
    }

    let pattern = Pattern { pat: b"longer".to_vec() }; // Pattern with 6 bytes
    let text = b"short"; // Text is shorter than the pattern, should not reach slice access
    assert!(pattern.is_suffix(text)); // This will cause a panic due to out of bounds access
}

