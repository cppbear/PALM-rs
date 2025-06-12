// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_limit_hit() {
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = regex::Regex::new("match").unwrap();
    let text = "this is a match and another match";
    let limit = 2;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    
    assert_eq!(result, std::borrow::Cow::Owned("this is a replacement and another match".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_exceeding_limit() {
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = regex::Regex::new("match").unwrap();
    let text = "match match match";
    let limit = 1;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    
    assert_eq!(result, std::borrow::Cow::Owned("replacement match match".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_and_empty_string() {
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = regex::Regex::new("match").unwrap();
    let text = "no match here";
    let limit = 5;
    let result = regex.replacen(text, limit, NoExpansionReplacer);
    
    assert_eq!(result, std::borrow::Cow::Borrowed("no match here".to_string()));
}

