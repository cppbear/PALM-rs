// Answer 0

#[test]
fn test_replacen_with_no_expansion() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {}
    }

    let replacer = TestReplacer;
    let text: &[u8] = b"This is a test. This will be replaced.";
    let limit = 1;

    let result = replacen(&replacer, text, limit, replacer);

    let expected: &[u8] = b"This is a REPLACED. This will be replaced.";
    assert_eq!(result, Cow::Owned(expected.to_vec()));
}

#[test]
fn test_replacen_with_limit_exceeds_matches() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {}
    }

    let replacer = TestReplacer;
    let text: &[u8] = b"This is a test. This will be replaced. This is another test.";
    let limit = 2;

    let result = replacen(&replacer, text, limit, replacer);

    let expected: &[u8] = b"This is a REPLACED. This will be REPLACED.";
    assert_eq!(result, Cow::Owned(expected.to_vec()));
}

#[test]
fn test_replacen_with_limit_equals_matches() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"REPLACED")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {}
    }

    let replacer = TestReplacer;
    let text: &[u8] = b"This is a test. Test is a test. Test.";
    let limit = 2;

    let result = replacen(&replacer, text, limit, replacer);

    let expected: &[u8] = b"This is a REPLACED. REPLACED is a test. Test.";
    assert_eq!(result, Cow::Owned(expected.to_vec()));
}

