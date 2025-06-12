// Answer 0

#[test]
fn test_replacen_no_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }

        fn replace_append(&self, _cap: &Captures, _output: &mut String) {
            // Not used in this test
        }
    }

    let regex = Regex::new("nonexistent").unwrap();
    let result = regex.replacen("Hello, world!", 0, DummyReplacer);
    assert_eq!(result, Cow::Borrowed("Hello, world!"));
}

#[test]
fn test_replacen_limit() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }

        fn replace_append(&self, _cap: &Captures, _output: &mut String) {
            // Not used in this test
        }
    }

    let regex = Regex::new("o").unwrap();
    let result = regex.replacen("Hello, world!", 1, DummyReplacer);
    assert_eq!(result, Cow::Owned("Hellreplacement, world!".to_string()));
}

#[test]
fn test_replacen_all_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("X")
        }

        fn replace_append(&self, _cap: &Captures, _output: &mut String) {
            // Not used in this test
        }
    }

    let regex = Regex::new("o").unwrap();
    let result = regex.replacen("Hello, world!", 0, DummyReplacer);
    assert_eq!(result, Cow::Owned("HellX, wXrld!".to_string()));
}

#[test]
fn test_replacen_no_expansion_needed() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("foo")
        }

        fn replace_append(&self, _cap: &Captures, _output: &mut String) {
            // Not used in this test
        }
    }

    let regex = Regex::new("bar").unwrap();
    let result = regex.replacen("bar bar bar", 0, DummyReplacer);
    assert_eq!(result, Cow::Owned("foo foo foo".to_string()));
}

