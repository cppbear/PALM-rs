// Answer 0

#[test]
fn test_replacen_no_expansion_no_matches() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"nonexistent").unwrap();
    let text = "this text has no matches";
    let replacer = DummyReplacer;

    let result = regex.replacen(text, 0, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_with_empty_string() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"nonexistent").unwrap();
    let text = ""; 
    let replacer = DummyReplacer;

    let result = regex.replacen(text, 0, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

