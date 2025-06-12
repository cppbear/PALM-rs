// Answer 0

#[test]
fn test_replacen_basic_case() {
    struct SimpleReplacer {
        replacement: String,
    }
    
    impl SimpleReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("foo").unwrap();
    let text = "foo bar foo baz";
    let replacer = SimpleReplacer {
        replacement: "bar".to_string(),
    };
    
    let result = regex.replacen(text, 1, replacer);
    assert_eq!(result, Cow::Owned("bar bar foo baz".to_string()));
}

#[test]
fn test_replacen_limit() {
    struct SimpleReplacer {
        replacement: String,
    }

    impl SimpleReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("foo").unwrap();
    let text = "foo bar foo baz";
    let replacer = SimpleReplacer {
        replacement: "bar".to_string(),
    };

    let result = regex.replacen(text, 1, replacer);
    assert_eq!(result, Cow::Owned("bar bar foo baz".to_string()));

    let result = regex.replacen(text, 0, replacer);
    assert_eq!(result, Cow::Owned("bar bar bar baz".to_string()));
}

#[test]
fn test_replacen_no_matches() {
    struct SimpleReplacer {
        replacement: String,
    }

    impl SimpleReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some(&self.replacement)
        }
    }

    let regex = Regex::new("non-existent").unwrap();
    let text = "foo bar baz";
    let replacer = SimpleReplacer {
        replacement: "bar".to_string(),
    };

    let result = regex.replacen(text, 1, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

