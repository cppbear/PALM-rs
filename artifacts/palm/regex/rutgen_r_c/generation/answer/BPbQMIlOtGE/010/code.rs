// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_matches() {
    struct MockReplacer {
        replacement: String,
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("test").unwrap();
    let text = "this is a test and another test here";
    let limit = 1;
    let replacer = MockReplacer {
        replacement: String::from("replace"),
    };
    
    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned("this is a replace and another test here".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    struct MockReplacer {
        replacement: String,
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("notfound").unwrap();
    let text = "this text has no matches";
    let limit = 5;
    let replacer = MockReplacer {
        replacement: String::from("replace"),
    };
    
    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_with_multiple_matches_and_limit() {
    struct MockReplacer {
        replacement: String,
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("test").unwrap();
    let text = "test string with multiple test appearances";
    let limit = 1;
    let replacer = MockReplacer {
        replacement: String::from("REPLACED"),
    };
    
    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned("REPLACED string with multiple test appearances".to_string()));
}

