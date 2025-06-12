// Answer 0

#[test]
fn test_replacen_no_expansion_with_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"_replacement_")
        }

        fn replace_append(&self, _cap: &Captures, _dst: &mut Vec<u8>) {
            // No-op for this test, focusing on no_expansion.
        }
    }

    let regex = Regex::new(r"foo").unwrap();
    let text = b"foo bar foo baz";
    let limit = 2;
    let rep = DummyReplacer;

    let result = regex.replacen(text, limit, rep);

    assert_eq!(result, Cow::Owned(b"_replacement_ bar _replacement_ baz".to_vec()));
}

#[test]
fn test_replacen_with_capture_groups() {
    struct CaptureReplacer;

    impl Replacer for CaptureReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            None
        }

        fn replace_append(&self, cap: &Captures, dst: &mut Vec<u8>) {
            if let Some(m) = cap.get(1) {
                dst.extend_from_slice(m.as_bytes());
            }
            dst.extend_from_slice(b" - modified");
        }
    }

    let regex = Regex::new(r"(?P<first>foo)").unwrap();
    let text = b"foo bar foo baz";
    let limit = 2;
    let rep = CaptureReplacer;

    let result = regex.replacen(text, limit, rep);

    assert_eq!(result, Cow::Owned(b"foo - modified bar foo - modified baz".to_vec()));
}

#[test]
fn test_replacen_no_matches() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"_replacement_")
        }

        fn replace_append(&self, _cap: &Captures, _dst: &mut Vec<u8>) {}
    }

    let regex = Regex::new(r"notfound").unwrap();
    let text = b"foo bar baz";
    let limit = 1;
    let rep = DummyReplacer;

    let result = regex.replacen(text, limit, rep);

    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_with_zero_limit() {
    struct DummyReplacer;

    impl Replacer for DummyReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"_replacement_")
        }

        fn replace_append(&self, _cap: &Captures, _dst: &mut Vec<u8>) {}
    }

    let regex = Regex::new(r"foo").unwrap();
    let text = b"foo bar foo baz";
    let limit = 0;
    let rep = DummyReplacer;

    let result = regex.replacen(text, limit, rep);

    assert_eq!(result, Cow::Owned(b"_replacement_ bar _replacement_ baz".to_vec()));
}

