// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replaced")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {}
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"no matches here.";
    let result = replacen(&replacer, text, 0, replacer);

    assert_eq!(result, Cow::Owned(b"no matches here.".to_vec()));
}

#[test]
fn test_replacen_with_no_expansion_and_some_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replaced")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {}
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"some matches here. some matches again.";
    
    // Simulating matches found by replacing the text
    let result = replacen(&replacer, text, 1, replacer);

    assert_eq!(result, Cow::Owned(b"replaced some matches again.".to_vec()));
}

#[test]
fn test_replacen_with_no_expansion_exceeding_limit() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replaced")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {}
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"some matches here. some matches here.";
    let result = replacen(&replacer, text, 1, replacer);

    assert_eq!(result, Cow::Owned(b"replaced some matches here.".to_vec()));
}

#[test]
#[should_panic]
fn test_replacen_panic_on_empty_text() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replaced")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {}
    }

    let replacer = DummyReplacer;
    let text: &[u8] = b"";
    
    // This should panic as the text is empty, thus no matches
    let _result = replacen(&replacer, text, 0, replacer);
}

