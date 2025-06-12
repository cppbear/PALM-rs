// Answer 0

#[test]
fn test_replacen_no_matches_with_no_expansion() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(b"replacement")
        }
    }

    let regex = Regex::new(r"nonexistent").unwrap();
    let text: &[u8] = b"no matches here";
    let limit = 0;
    let rep = DummyReplacer;

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_matches_with_no_expansion_and_limit() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(b"replacement")
        }
    }

    let regex = Regex::new(r"nonexistent").unwrap();
    let text: &[u8] = b"nothing to replace";
    let limit = 5;
    let rep = DummyReplacer;

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_empty_text_with_no_expansion() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(b"replacement")
        }
    }

    let regex = Regex::new(r"nonexistent").unwrap();
    let text: &[u8] = b"";
    let limit = 0;
    let rep = DummyReplacer;

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Borrowed(text));
}

