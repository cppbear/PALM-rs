// Answer 0

#[test]
fn test_replacen_no_expansion_empty_text() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").expect("Failed to create regex");
    let text = "";
    let limit = 1;
    let rep = NoExpansionReplacer;

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Borrowed(""));
}

#[test]
fn test_replacen_no_expansion_one_match() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").expect("Failed to create regex");
    let text = "a";
    let limit = 1;
    let rep = NoExpansionReplacer;

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Owned("replacement".to_string()));
}

#[test]
fn test_replacen_no_expansion_two_matches_limit_one() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").expect("Failed to create regex");
    let text = "aaa";
    let limit = 1;
    let rep = NoExpansionReplacer;

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Owned("replacementaa".to_string()));
}

#[test]
fn test_replacen_no_expansion_two_matches_limit_two() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").expect("Failed to create regex");
    let text = "aaa";
    let limit = 2;
    let rep = NoExpansionReplacer;

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Owned("replacementreplacement".to_string()));
}

#[test]
#[should_panic]
fn test_replacen_no_expansion_edge_case_last_match() {
    struct NoExpansionReplacer;

    impl NoExpander for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").expect("Failed to create regex");
    let text = "aaa";
    let limit = 3;
    let rep = NoExpansionReplacer;

    let result = regex.replacen(text, limit, rep);
    // This should panic as last_match will go out of bounds
    assert!(result.is_ok());
}

