// Answer 0

#[test]
fn test_replace_all_empty_text() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, cap: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"replacement")
        }
    }

    let re = Regex::new(r"pattern").unwrap();
    let result = re.replace_all(b"", DummyReplacer);
    assert_eq!(result, b"");
}

#[test]
fn test_replace_all_no_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, cap: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"replacement")
        }
    }

    let re = Regex::new(r"pattern").unwrap();
    let result = re.replace_all(b"no match here", DummyReplacer);
    assert_eq!(result, b"no match here");
}

#[test]
fn test_replace_all_with_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, cap: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"replacement")
        }
    }

    let re = Regex::new(r"test").unwrap();
    let result = re.replace_all(b"this is a test. Let's test it.", DummyReplacer);
    assert_eq!(result, b"this is a replacement. Let's replacement it.");
}

#[test]
fn test_replace_all_boundary_conditions() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, cap: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"replacement")
        }
    }

    let re = Regex::new(r"abc").unwrap();
    let result = re.replace_all(b"abcabcabc", DummyReplacer);
    assert_eq!(result, b"replacementreplacementreplacement");
}

