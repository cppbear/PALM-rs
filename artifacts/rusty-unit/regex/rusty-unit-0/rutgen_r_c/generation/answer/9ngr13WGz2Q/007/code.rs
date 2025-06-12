// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_none_found() {
    struct NoExpansionReplacer;

    impl NoExpansion for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"nonexistent").unwrap();
    let text = b"some sample text";
    let limit = 3;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_with_no_expansion_and_some_found() {
    struct NoExpansionReplacer;

    impl NoExpansion for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"sample").unwrap();
    let text = b"some sample text sample";
    let limit = 1;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Owned(b"some REPLACED text sample".to_vec()));
}

#[test]
fn test_replacen_with_limit_reached() {
    struct NoExpansionReplacer;

    impl NoExpansion for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"sample").unwrap();
    let text = b"some sample text sample";
    let limit = 1; // Limit set to 1
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Owned(b"some REPLACED text sample".to_vec()));
}

#[test]
fn test_replacen_with_multiple_occurrences() {
    struct NoExpansionReplacer;

    impl NoExpansion for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }
    }

    let regex = Regex::new(r"sample").unwrap();
    let text = b"sample sample sample";
    let limit = 2; // Limit set to 2
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Owned(b"REPLACED REPLACED sample".to_vec()));
}

