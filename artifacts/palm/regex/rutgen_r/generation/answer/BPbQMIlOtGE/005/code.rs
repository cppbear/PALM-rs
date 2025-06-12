// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    use std::borrow::Cow;

    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let replacer = DummyReplacer;
    let text = "Hello, world!";
    let limit = 0; // Replace all occurrences

    let result = replacer.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned("replacement".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_some_matches() {
    use std::borrow::Cow;

    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("x")
        }
    }

    let replacer = DummyReplacer;
    let text = "a b c d a";
    let limit = 2; // Limit replacements to the first 2 occurrences

    let result = replacer.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned("x b c d x".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_limit_zero() {
    use std::borrow::Cow;

    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replace")
        }
    }

    let replacer = DummyReplacer;
    let text = "replace replace replace";
    let limit = 0; // Should replace all occurrences

    let result = replacer.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned("replace replace replace".to_string()));
}

#[test]
#[should_panic]
fn test_replacen_panic_due_to_empty_text() {
    use std::borrow::Cow;

    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("x")
        }
    }

    let replacer = DummyReplacer;
    let text = "";
    let limit = 1;

    // This call should panic since text is empty and last_match will cause a panic
    let _ = replacer.replacen(text, limit, replacer);
}

