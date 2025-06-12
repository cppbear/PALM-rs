// Answer 0

#[test]
fn test_replace_all_no_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("abc").unwrap();
    let result = regex.replace_all("def", DummyReplacer);
    assert_eq!(result, Cow::Borrowed("def"));
}

#[test]
fn test_replace_all_with_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new("a").unwrap();
    let result = regex.replace_all("abcabc", DummyReplacer);
    assert_eq!(result, Cow::Owned("XbcXbc".to_string()));
}

#[test]
fn test_replace_all_multiple_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("Y")
        }
    }

    let regex = Regex::new("b").unwrap();
    let result = regex.replace_all("abcabc", DummyReplacer);
    assert_eq!(result, Cow::Owned("aYcaYca".to_string()));
}

