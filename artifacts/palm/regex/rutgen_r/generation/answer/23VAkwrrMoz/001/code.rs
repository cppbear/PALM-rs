// Answer 0

#[test]
fn test_approximate_size_empty_pattern() {
    struct Pattern {
        pat: String,
    }

    impl Pattern {
        fn approximate_size(&self) -> usize {
            self.pat.len() * std::mem::size_of::<u8>()
        }
    }

    let pattern = Pattern { pat: String::new() };
    assert_eq!(pattern.approximate_size(), 0);
}

#[test]
fn test_approximate_size_single_character_pattern() {
    struct Pattern {
        pat: String,
    }

    impl Pattern {
        fn approximate_size(&self) -> usize {
            self.pat.len() * std::mem::size_of::<u8>()
        }
    }

    let pattern = Pattern { pat: String::from("a") };
    assert_eq!(pattern.approximate_size(), 1);
}

#[test]
fn test_approximate_size_multiple_characters_pattern() {
    struct Pattern {
        pat: String,
    }

    impl Pattern {
        fn approximate_size(&self) -> usize {
            self.pat.len() * std::mem::size_of::<u8>()
        }
    }

    let pattern = Pattern { pat: String::from("abcde") };
    assert_eq!(pattern.approximate_size(), 5);
}

#[test]
fn test_approximate_size_long_pattern() {
    struct Pattern {
        pat: String,
    }

    impl Pattern {
        fn approximate_size(&self) -> usize {
            self.pat.len() * std::mem::size_of::<u8>()
        }
    }

    let pattern = Pattern { pat: String::from("this is a longer pattern") };
    assert_eq!(pattern.approximate_size(), 27);
}

#[test]
fn test_approximate_size_non_ascii_characters() {
    struct Pattern {
        pat: String,
    }

    impl Pattern {
        fn approximate_size(&self) -> usize {
            self.pat.len() * std::mem::size_of::<u8>()
        }
    }

    let pattern = Pattern { pat: String::from("ñöë") };
    assert_eq!(pattern.approximate_size(), 4);
}

