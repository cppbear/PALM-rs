// Answer 0

#[test]
fn test_replacen_no_expansion_empty() {
    struct NoExpansion;

    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let text = "";
    let limit = 0;
    let replacer = NoExpansion;
    
    let result = replacen(&replacer, text, limit, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_no_matches() {
    struct NoExpansion;

    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let text = "no matches here";
    let limit = 0;
    let replacer = NoExpansion;

    let result = replacen(&replacer, text, limit, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_limit_zero() {
    struct NoExpansion;

    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let text = "more text without any matching pattern";
    let limit = 0;
    let replacer = NoExpansion;

    let result = replacen(&replacer, text, limit, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

