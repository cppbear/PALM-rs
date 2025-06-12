// Answer 0

#[test]
fn test_approximate_size() {
    struct Pattern {
        pat: String,
    }

    impl Pattern {
        fn approximate_size(&self) -> usize {
            self.pat.len() * std::mem::size_of::<u8>()
        }
    }

    let pattern = Pattern {
        pat: String::from("abc"),
    };

    let size = pattern.approximate_size();
    assert_eq!(size, 3 * std::mem::size_of::<u8>());
}

#[test]
fn test_approximate_size_empty() {
    struct Pattern {
        pat: String,
    }

    impl Pattern {
        fn approximate_size(&self) -> usize {
            self.pat.len() * std::mem::size_of::<u8>()
        }
    }

    let pattern = Pattern {
        pat: String::from(""),
    };

    let size = pattern.approximate_size();
    assert_eq!(size, 0);
}

