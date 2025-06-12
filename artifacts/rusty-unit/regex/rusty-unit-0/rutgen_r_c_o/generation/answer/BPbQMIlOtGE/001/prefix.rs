// Answer 0

#[test]
fn test_replacen_empty_text() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("abc").unwrap();
    let result = regex.replacen("", 0, DummyReplacer);
}

#[test]
fn test_replacen_non_empty_text_zero_limit() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("abc").unwrap();
    let result = regex.replacen("any string", 0, DummyReplacer);
}

#[test]
fn test_replacen_non_empty_text_one_limit() {
    struct DummyReplacer;

    impl DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("abc").unwrap();
    let result = regex.replacen("any string", 1, DummyReplacer);
}

