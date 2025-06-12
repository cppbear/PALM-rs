// Answer 0

#[test]
fn test_replacen_no_expansion_empty_text() {
    #[derive(Default)]
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").unwrap();
    let text = "";
    let limit = 0;
    let replacer = NoExpansionReplacer::default();

    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_no_matches() {
    #[derive(Default)]
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").unwrap();
    let text = "hello world";
    let limit = 0;
    let replacer = NoExpansionReplacer::default();

    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_expansion_with_limit() {
    #[derive(Default)]
    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("a").unwrap();
    let text = "banana";
    let limit = 1; // Replace only one match
    let replacer = NoExpansionReplacer::default();

    let result = regex.replacen(text, limit, replacer);
    assert_eq!(result, Cow::Owned("breplacementnana".to_string()));
}

