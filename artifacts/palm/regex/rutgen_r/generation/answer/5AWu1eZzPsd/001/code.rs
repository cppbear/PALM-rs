// Answer 0

#[test]
fn test_replace_all_with_empty_input() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, _captures: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"")
        }
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"";
    let result = replace_all(text, replacer);
    assert_eq!(result, Cow::Borrowed(b""));
}

#[test]
fn test_replace_all_with_no_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, _captures: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"")
        }
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"no match here";
    let result = replace_all(text, replacer);
    assert_eq!(result, Cow::Borrowed(b"no match here"));
}

#[test]
fn test_replace_all_with_single_match() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, _captures: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"replacement")
        }
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"match replacement";
    let result = replace_all(text, replacer);
    assert_eq!(result, Cow::Borrowed(b"replacement replacement"));
}

#[test]
fn test_replace_all_with_multiple_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn replace(&self, _captures: &Captures) -> Cow<[u8]> {
            Cow::Borrowed(b"replacement")
        }
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"match replacement and another match replacement";
    let result = replace_all(text, replacer);
    assert_eq!(result, Cow::Borrowed(b"replacement replacement and another replacement"));
}

#[test]
#[should_panic]
fn test_replace_all_with_panic_condition() {
    struct PanicReplacer;

    impl Replacer for PanicReplacer {
        fn replace(&self, _captures: &Captures) -> Cow<[u8]> {
            panic!("This should trigger a panic");
        }
    }

    let replacer = PanicReplacer;
    let text: &[u8] = b"this will cause panic";
    let _ = replace_all(text, replacer);
}

