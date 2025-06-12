// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    use std::borrow::Cow;

    struct NoExpansionReplacer;

    impl NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            Some(b"REPLACEMENT")
        }
    }

    impl Replacer for NoExpansionReplacer {}

    let replacer = NoExpansionReplacer;
    let text: &[u8] = b"no matches here";
    let limit: usize = 3; // arbitrary limit, won't matter as there are no matches

    let result = replacen(&replacer, text, limit, replacer);
    
    assert_eq!(result, Cow::Borrowed(text));
}

