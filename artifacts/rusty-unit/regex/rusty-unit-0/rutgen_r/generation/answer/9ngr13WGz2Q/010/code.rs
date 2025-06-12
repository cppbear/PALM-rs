// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    struct MockReplacer {
        replacement: &'static [u8],
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(self.replacement)
        }
    }

    let replacer = MockReplacer {
        replacement: b"replacement"
    };
    let text = b"no matches here";
    let limit = 0;

    let result = replacen(&replacer, text, limit, replacer);
    let expected: Vec<u8> = b"no matches here".to_vec();

    assert_eq!(result, Cow::Owned(expected));
}

#[test]
fn test_replacen_with_no_expansion_and_some_matches() {
    struct MockReplacer {
        replacement: &'static [u8],
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(self.replacement)
        }
    }

    let replacer = MockReplacer {
        replacement: b"replacement"
    };
    let text = b"replace this replace that";
    let limit = 2;

    // Simulated matches (assume they exist in self's find_iter)
    let expected: Vec<u8> = b"replacement this replacement that".to_vec();

    let result = replacen(&replacer, text, limit, replacer);
    assert_eq!(result, Cow::Owned(expected));
}

#[test]
fn test_replacen_with_limit_zero_and_no_expansion() {
    struct MockReplacer {
        replacement: &'static [u8],
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(self.replacement)
        }
    }

    let replacer = MockReplacer {
        replacement: b"replacement"
    };
    let text = b"replace this replace that";
    let limit = 0; // all matches should be replaced regardless of count

    // Simulated scenario where every target is replaced
    let expected: Vec<u8> = b"replacement this replacement that".to_vec();

    let result = replacen(&replacer, text, limit, replacer);
    assert_eq!(result, Cow::Owned(expected));
}

