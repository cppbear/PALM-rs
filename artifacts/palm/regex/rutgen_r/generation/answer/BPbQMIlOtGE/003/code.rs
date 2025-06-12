// Answer 0

#[test]
fn test_replacen_with_no_expansion() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new("foo").unwrap();
    let text = "foo bar foo";
    let limit = 2;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Owned("X bar X".to_string()));
}

#[test]
fn test_replacen_with_no_matches() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new("baz").unwrap();
    let text = "foo bar baz";
    let limit = 1;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_exceeding_limit() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new("foo").unwrap();
    let text = "foo bar foo baz foo";
    let limit = 2;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Owned("X bar X baz foo".to_string()));
}

#[test]
fn test_replacen_with_multiple_matches() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new("bar").unwrap();
    let text = "foo bar baz bar qux";
    let limit = 1;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    assert_eq!(result, Cow::Owned("foo X baz bar qux".to_string()));
}

#[test]
#[should_panic]
fn test_replacen_panic_on_capture_get() {
    struct CaptureReplacer;

    impl Replacer for CaptureReplacer {
        fn replace_append(&self, cap: &Captures, dst: &mut String) {
            // intentionally causing panic
            let _ = cap.get(1).unwrap();
        }
    }

    let regex = Regex::new(r"(\d+)").unwrap();
    let text = "No digits here";
    let limit = 1;
    let _ = regex.replacen(text, limit, CaptureReplacer);
}

#[test]
#[should_panic]
fn test_replacen_panic_on_text_slice() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new("bar").unwrap();
    let text = "foo baz";
    let limit = 1;
    let _ = regex.replacen(text, limit, NoExpansionReplacer); // should not panic as there are no matches.
}

