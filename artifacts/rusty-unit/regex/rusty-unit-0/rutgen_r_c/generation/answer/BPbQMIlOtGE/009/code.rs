// Answer 0

#[test]
fn test_replacen_no_expansion_empty_text() {
    struct NoExpansion;
    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("REPLACED")
        }
        fn replace_append(&self, _captures: &Captures, _dst: &mut String) {}
    }

    let regex = Regex::new("foo").unwrap();
    let result = regex.replacen("", 0, NoExpansion);
    assert_eq!(result, Cow::Owned("".to_string()));
}

#[test]
fn test_replacen_no_expansion_no_matches() {
    struct NoExpansion;
    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("REPLACED")
        }
        fn replace_append(&self, _captures: &Captures, _dst: &mut String) {}
    }

    let regex = Regex::new("foo").unwrap();
    let result = regex.replacen("bar", 0, NoExpansion);
    assert_eq!(result, Cow::Owned("bar".to_string()));
}

#[test]
fn test_replacen_no_expansion_with_matches_limit_zero() {
    struct NoExpansion;
    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("REPLACED")
        }
        fn replace_append(&self, _captures: &Captures, _dst: &mut String) {}
    }

    let regex = Regex::new("foo").unwrap();
    let result = regex.replacen("foobarfoo", 0, NoExpansion);
    assert_eq!(result, Cow::Owned("REPLACEDbarREPLACED".to_string()));
}

#[test]
#[should_panic]
fn test_replacen_may_panic_on_empty_match() {
    struct NoExpansion;
    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("REPLACED")
        }
        fn replace_append(&self, _captures: &Captures, _dst: &mut String) {}
    }

    let regex = Regex::new("").unwrap(); // Match empty string
    let _ = regex.replacen("test", 0, NoExpansion); // Should panic due to index out of bounds
}

#[test]
fn test_replacen_no_expansion_panic_on_last_match() {
    struct NoExpansion;
    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("REPLACED")
        }
        fn replace_append(&self, _captures: &Captures, _dst: &mut String) {}
    }

    let regex = Regex::new("foo").unwrap();
    let result = regex.replacen("foo", 0, NoExpansion);
    assert_eq!(result, Cow::Owned("REPLACED".to_string())); // Expect to handle replacement correctly
}

