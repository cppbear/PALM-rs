// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_matching_text() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"\bfoo\b").unwrap();
    let text = "foo bar foo baz";
    let replacer = DummyReplacer;

    let result = regex.replacen(text, 2, replacer);
    assert_eq!(result, Cow::Owned("replacement bar replacement baz".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"\bxyz\b").unwrap();
    let text = "foo bar baz";
    let replacer = DummyReplacer;

    let result = regex.replacen(text, 2, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_with_no_expansion_and_exact_number_of_matches() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"\bfoo\b").unwrap();
    let text = "foo foo foo";
    let replacer = DummyReplacer;

    let result = regex.replacen(text, 3, replacer);
    assert_eq!(result, Cow::Owned("replacement replacement replacement".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_limit_exceeding_matches() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"\bfoo\b").unwrap();
    let text = "foo foo foo";
    let replacer = DummyReplacer;

    let result = regex.replacen(text, 5, replacer);
    assert_eq!(result, Cow::Owned("replacement replacement replacement".to_string()));
}

#[test]
fn test_replacen_with_expansion_and_single_match() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            None
        }

        fn replace_append(&self, _cap: &Captures, dst: &mut String) {
            dst.push_str("expanded");
        }
    }

    let regex = Regex::new(r"\bfoo\b").unwrap();
    let text = "foo bar";
    let replacer = DummyReplacer;

    let result = regex.replacen(text, 1, replacer);
    assert_eq!(result, Cow::Owned("expanded bar".to_string()));
}

