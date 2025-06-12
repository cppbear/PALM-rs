// Answer 0

#[test]
fn test_replacen_no_expansion_some_matches_with_limit() {
    struct DummyReplacer<'a>(&'a str);

    impl<'a> Replacer for DummyReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }

        fn replace_append(&self, _: &Captures, _: &mut String) {
            // This method is unused in this test since no expansion is used
        }
    }

    let dummy_replacer = DummyReplacer("REPLACED");
    let regex = Regex::new(r"\d+").unwrap(); // Regex that matches one or more digits
    let text = "123 456 789";
    let limit = 2;

    let result = regex.replacen(text, limit, dummy_replacer);

    assert_eq!(result, Cow::Owned("REPLACED REPLACED 789".to_string()));
}

#[test]
fn test_replacen_no_expansion_no_matches() {
    struct DummyReplacer<'a>(&'a str);

    impl<'a> Replacer for DummyReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }

        fn replace_append(&self, _: &Captures, _: &mut String) {
            // This method is unused in this test since no expansion is used
        }
    }

    let dummy_replacer = DummyReplacer("REPLACED");
    let regex = Regex::new(r"\d+").unwrap(); // Regex that matches one or more digits
    let text = "abc def ghi";
    let limit = 3;

    let result = regex.replacen(text, limit, dummy_replacer);

    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_exact_limit() {
    struct DummyReplacer<'a>(&'a str);

    impl<'a> Replacer for DummyReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }

        fn replace_append(&self, _: &Captures, _: &mut String) {
            // This method is unused in this test since no expansion is used
        }
    }

    let dummy_replacer = DummyReplacer("REPLACED");
    let regex = Regex::new(r"\d+").unwrap(); // Regex that matches one or more digits
    let text = "123 456";
    let limit = 2;

    let result = regex.replacen(text, limit, dummy_replacer);

    assert_eq!(result, Cow::Owned("REPLACED REPLACED".to_string()));
}

#[test]
#[should_panic]
fn test_replacen_last_match_panic() {
    struct DummyReplacer<'a>(&'a str);

    impl<'a> Replacer for DummyReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }

        fn replace_append(&self, _: &Captures, _: &mut String) {
            // This method is unused in this test since no expansion is used
        }
    }

    let dummy_replacer = DummyReplacer("REPLACED");
    let regex = Regex::new(r"\d+").unwrap(); // Regex that matches one or more digits
    let text = "123 456";
    let limit = 5;  // Limit is larger than matches available

    let _ = regex.replacen(text, limit, dummy_replacer);
}

