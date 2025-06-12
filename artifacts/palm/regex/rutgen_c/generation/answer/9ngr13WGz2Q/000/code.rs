// Answer 0

#[test]
fn test_replacen_no_expansion_limit_zero() {
    struct DummyReplacer;

    impl re_trait::Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _: &Captures, dst: &mut Vec<u8>) {
            // Not used for this test
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let text = b"Searching for 123 and 456.";
    let result = regex.replacen(text, 0, DummyReplacer);
    assert_eq!(result, Cow::Owned(b"Searching for REPLACED and REPLACED.".to_vec()));
}

#[test]
fn test_replacen_with_expansion() {
    struct ExpandingReplacer;

    impl re_trait::Replacer for ExpandingReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            None
        }

        fn replace_append(&self, caps: &Captures, dst: &mut Vec<u8>) {
            if let Some(m) = caps.get(0) {
                dst.extend_from_slice(b"EXPANDED(");
                dst.extend_from_slice(m.as_bytes());
                dst.extend_from_slice(b")");
            }
        }
    }

    let regex = Regex::new(r"(\d+)").unwrap();
    let text = b"Numbers: 1, 2, and 3.";
    let result = regex.replacen(text, 2, ExpandingReplacer);
    assert_eq!(result, Cow::Owned(b"Numbers: EXPANDED(1) , EXPANDED(2) , and 3.".to_vec()));
}

#[test]
fn test_replacen_with_limit() {
    struct DummyReplacer;

    impl re_trait::Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _: &Captures, dst: &mut Vec<u8>) {}
    }

    let regex = Regex::new(r"\d+").unwrap();
    let text = b"1 2 3";
    let result = regex.replacen(text, 2, DummyReplacer);
    assert_eq!(result, Cow::Owned(b"REPLACED REPLACED 3".to_vec()));
}

#[test]
fn test_replacen_with_no_matches() {
    struct DummyReplacer;

    impl re_trait::Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _: &Captures, dst: &mut Vec<u8>) {}
    }

    let regex = Regex::new(r"\d+").unwrap();
    let text = b"No digits here.";
    let result = regex.replacen(text, 0, DummyReplacer);
    assert_eq!(result, Cow::Borrowed(text));
}

