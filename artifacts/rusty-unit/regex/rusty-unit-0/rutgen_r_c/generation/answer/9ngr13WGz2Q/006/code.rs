// Answer 0

#[test]
fn test_replacen_no_expansion_no_matches() {
    struct DummyReplacer;

    impl re_trait::Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }

        fn replace_append(&self, _captures: &Captures, _dst: &mut Vec<u8>) {
            // No-op since no expansion is required
        }
    }

    let regex = Regex::new(r"abc").unwrap();
    let text = b"xyz"; // No matches
    let limit = 0; // All matches to be replaced

    let result = regex.replacen(text, limit, DummyReplacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_some_matches() {
    struct DummyReplacer;

    impl re_trait::Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }

        fn replace_append(&self, _captures: &Captures, _dst: &mut Vec<u8>) {
            // No-op since no expansion is required
        }
    }

    let regex = Regex::new(r"abc").unwrap();
    let text = b"abc xyz abc"; // Two matches
    let limit = 1; // Limit to one replacement

    let result = regex.replacen(text, limit, DummyReplacer);
    let expected = b"replacement xyz abc"; // Only the first match should be replaced
    
    assert_eq!(result, Cow::Owned(expected.to_vec()));
}

#[test]
fn test_replacen_no_expansion_with_limit_zero() {
    struct DummyReplacer;

    impl re_trait::Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }

        fn replace_append(&self, _captures: &Captures, _dst: &mut Vec<u8>) {
            // No-op since no expansion is required
        }
    }

    let regex = Regex::new(r"abc").unwrap();
    let text = b"abc abc abc"; // Three matches
    let limit = 0; // Replace all matches

    let result = regex.replacen(text, limit, DummyReplacer);
    let expected = b"replacement replacement replacement"; // All matches should be replaced

    assert_eq!(result, Cow::Owned(expected.to_vec()));
}

