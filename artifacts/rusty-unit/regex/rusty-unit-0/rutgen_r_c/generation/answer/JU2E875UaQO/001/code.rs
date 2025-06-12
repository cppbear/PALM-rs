// Answer 0

#[test]
fn test_replace_all_empty_string() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> { Some("X") }
    }

    let regex = Regex::new(r"abc").unwrap();
    let result = regex.replace_all("", DummyReplacer);
    assert_eq!(result, Cow::Borrowed(""));
}

#[test]
fn test_replace_all_no_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> { Some("X") }
    }

    let regex = Regex::new(r"abc").unwrap();
    let result = regex.replace_all("def", DummyReplacer);
    assert_eq!(result, Cow::Borrowed("def"));
}

#[test]
fn test_replace_all_single_match() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> { Some("X") }
    }

    let regex = Regex::new(r"abc").unwrap();
    let result = regex.replace_all("abc def", DummyReplacer);
    assert_eq!(result, Cow::Owned("X def".to_string()));
}

#[test]
fn test_replace_all_multiple_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> { Some("X") }
    }

    let regex = Regex::new(r"abc").unwrap();
    let result = regex.replace_all("abc abc def", DummyReplacer);
    assert_eq!(result, Cow::Owned("X X def".to_string()));
}

#[test]
fn test_replace_all_with_captures() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace_append(&self, captures: &Captures, result: &mut String) {
            if let Some(mat) = captures.get(0) {
                result.push_str(&format!("{}-replaced", mat.as_str()));
            }
        }
    }

    let regex = Regex::new(r"(abc)").unwrap();
    let result = regex.replace_all("abc def abc", DummyReplacer);
    assert_eq!(result, Cow::Owned("abc-replaced def abc-replaced".to_string()));
}

