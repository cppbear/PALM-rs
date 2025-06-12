// Answer 0

#[test]
fn test_replacen_with_no_matches() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }
    }

    let replacer = MockReplacer;
    let result = replacer.replacen(b"no matches here", 0, replacer);
    assert_eq!(result, Cow::Borrowed(b"no matches here"));
}

#[test]
fn test_replacen_with_limit() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }
    }

    let replacer = MockReplacer;
    let result = replacer.replacen(b"test test test", 1, replacer);
    assert_eq!(result, Cow::Owned(b"replacement test test".to_vec()));
}

#[test]
fn test_replacen_with_zero_limit() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }
    }

    let replacer = MockReplacer;
    let result = replacer.replacen(b"test test test", 0, replacer);
    assert_eq!(result, Cow::Owned(b"replacement replacement replacement".to_vec()));
}

#[test]
fn test_replacen_with_captures() {
    struct MockReplacer;

    impl Replacer for MockReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            None
        }
        
        fn replace_append(&self, _cap: &Capture, new: &mut Vec<u8>) {
            new.extend_from_slice(b"captured");
        }
    }

    let replacer = MockReplacer;
    let result = replacer.replacen(b"match this match that", 2, replacer);
    assert_eq!(result, Cow::Owned(b"captured match that".to_vec()));
}

