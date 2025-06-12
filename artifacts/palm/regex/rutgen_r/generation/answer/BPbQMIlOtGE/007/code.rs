// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    struct MockReplacer;

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    let replacer = MockReplacer;
    let text = "This is a test string.";
    let limit = 1;

    let result = replacen(&replacer, text, limit, replacer);

    assert_eq!(result, Cow::Owned("replacement test string.".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_one_match() {
    struct MockReplacer;

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    impl Regex {
        fn find_iter<'t>(&self, text: &'t str) -> Box<dyn Iterator<Item = Match<'t>> + 't> {
            // Mock implementation returning a single match
            Box::new(vec![Match::new(10, 20)].into_iter())
        }
    }

    let replacer = MockReplacer;
    let text = "This is a test string with a match.";
    let limit = 1;

    let result = replacen(&replacer, text, limit, replacer);

    assert_eq!(result, Cow::Owned("This is a replacement string with a match.".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_limit_exceeded() {
    struct MockReplacer;

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    impl Regex {
        fn find_iter<'t>(&self, text: &'t str) -> Box<dyn Iterator<Item = Match<'t>> + 't> {
            // Mock implementation returning two matches
            Box::new(vec![Match::new(10, 20), Match::new(30, 40)].into_iter())
        }
    }

    let replacer = MockReplacer;
    let text = "This is a test string with a match.";
    let limit = 1;

    let result = replacen(&replacer, text, limit, replacer);

    assert_eq!(result, Cow::Owned("This is a replacement string with a match.".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_multiple_replacements() {
    struct MockReplacer;

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    impl Regex {
        fn find_iter<'t>(&self, text: &'t str) -> Box<dyn Iterator<Item = Match<'t>> + 't> {
            // Mock implementation simulating multiple matches
            Box::new(vec![Match::new(10, 20), Match::new(30, 40), Match::new(50, 60)].into_iter())
        }
    }

    let replacer = MockReplacer;
    let text = "aaa aaa aaa.";
    let limit = 2; // Set limit to 2 to test replacement behavior

    let result = replacen(&replacer, text, limit, replacer);

    assert_eq!(result, Cow::Owned("aaa replacement aaa.".to_string()));
}

