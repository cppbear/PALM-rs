// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _cap: &Captures, _vec: &mut Vec<u8>) {}
    }

    let dummy = DummyReplacer;
    let text = b"abc def ghi abc def";
    let limit = 0; // Replace all matches

    let result = dummy.replacen(text, limit, dummy);
    assert_eq!(result, Cow::Owned(b"REPLACED REPLACED".to_vec()));
}

#[test]
#[should_panic]
fn test_replacen_with_no_matches_and_nonzero_limit() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _cap: &Captures, _vec: &mut Vec<u8>) {}
    }

    let dummy = DummyReplacer;
    let text = b"";
    let limit = 5; // Limit is non-zero

    let _result = dummy.replacen(text, limit, dummy);
}

#[test]
fn test_replacen_with_panics_on_empty_text_after_match() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            None
        }

        fn replace_append(&self, _cap: &Captures, _vec: &mut Vec<u8>) {
            // Just a dummy implementation for the purpose of the test
        }
    }

    let dummy = DummyReplacer;
    let text = b"abc";
    let limit = 0; // Replaces all matches

    let result = dummy.replacen(text, limit, dummy);
    assert_eq!(result, Cow::Borrowed(text));
}

