// Answer 0

#[test]
fn test_replacen_with_no_expansion() {
    struct NoExpansion;

    impl NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"\d+").unwrap(); // Assume regex matches digits
    let text = "123 and 456";
    let limit = 1;

    let result = regex.replacen(text, limit, NoExpansion);

    assert_eq!(result, Cow::Owned("replacement and 456".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_multiple_replacements() {
    struct NoExpansion;

    impl NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let text = "123 and 456 and 789";
    let limit = 2;

    let result = regex.replacen(text, limit, NoExpansion);

    assert_eq!(result, Cow::Owned("X and X and 789".to_string()));
}

#[test]
fn test_replacen_no_matches() {
    struct NoExpansion;

    impl NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let text = "no digits here";
    let limit = 1;

    let result = regex.replacen(text, limit, NoExpansion);

    assert_eq!(result, Cow::Borrowed("no digits here"));
}

#[test]
fn test_replacen_empty_text() {
    struct NoExpansion;

    impl NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let text = "";
    let limit = 1;

    let result = regex.replacen(text, limit, NoExpansion);

    assert_eq!(result, Cow::Borrowed(""));
}

#[test]
fn test_replacen_limit_zero() {
    struct NoExpansion;

    impl NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let text = "123 and 456";
    let limit = 0;

    let result = regex.replacen(text, limit, NoExpansion);

    assert_eq!(result, Cow::Owned("replacement and replacement".to_string()));
}

