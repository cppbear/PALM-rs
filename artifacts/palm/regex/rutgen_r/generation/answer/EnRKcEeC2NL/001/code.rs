// Answer 0

#[test]
fn test_len_non_empty_pattern() {
    struct Pattern {
        pattern: String,
    }

    impl Pattern {
        fn len(&self) -> usize {
            return self.pattern.len();
        }
    }

    let p = Pattern {
        pattern: String::from("abcde"),
    };
    assert_eq!(p.len(), 5);
}

#[test]
fn test_len_empty_pattern() {
    struct Pattern {
        pattern: String,
    }

    impl Pattern {
        fn len(&self) -> usize {
            return self.pattern.len();
        }
    }

    let p = Pattern {
        pattern: String::from(""),
    };
    assert_eq!(p.len(), 0);
}

#[test]
fn test_len_whitespace_pattern() {
    struct Pattern {
        pattern: String,
    }

    impl Pattern {
        fn len(&self) -> usize {
            return self.pattern.len();
        }
    }

    let p = Pattern {
        pattern: String::from("    "), // String with only whitespace
    };
    assert_eq!(p.len(), 4);
}

#[test]
fn test_len_special_characters() {
    struct Pattern {
        pattern: String,
    }

    impl Pattern {
        fn len(&self) -> usize {
            return self.pattern.len();
        }
    }

    let p = Pattern {
        pattern: String::from("!@#$%^&*()"),
    };
    assert_eq!(p.len(), 10);
}

