// Answer 0

#[test]
fn test_replacen_no_expansion_empty_text() {
    struct SimpleReplacer;

    impl Replacer for SimpleReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {
            // No-op since no expansion is involved
        }
    }

    let replacer = SimpleReplacer;
    let text: &[u8] = b"";
    let limit = 1;

    let result = replacen(&replacer, text, limit, replacer);
    let expected = Cow::Borrowed(text);

    assert_eq!(result, expected);
}

#[test]
fn test_replacen_no_expansion_no_matches() {
    struct SimpleReplacer;

    impl Replacer for SimpleReplacer {
        fn no_expansion(&self) -> Option<&[u8]> {
            Some(b"replacement")
        }

        fn replace_append(&self, _cap: &Captures, _new: &mut Vec<u8>) {
            // No-op since no expansion is involved
        }
    }

    let replacer = SimpleReplacer;
    let text: &[u8] = b"no matches here";
    let limit = 1;

    let result = replacen(&replacer, text, limit, replacer);
    let expected = Cow::Borrowed(text);

    assert_eq!(result, expected);
}

