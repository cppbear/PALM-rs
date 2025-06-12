// Answer 0

#[test]
fn test_replacen_no_matches_with_no_expansion() {
    struct NoExpansionReplacer;

    impl Replacer for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
        
        fn replace_append(&self, _cap: &Captures, _output: &mut String) {
            // No operation needed for no expansion
        }
    }

    let regex = Regex::new(r"abc").unwrap(); // Example regex that doesn't match
    let text = "defghi";
    let limit = 0; // All non-overlapping matches should be replaced

    let result = regex.replacen(text, limit, NoExpansionReplacer);
    
    assert_eq!(result, Cow::Borrowed(text));
}

#[test]
fn test_replacen_no_matches_with_no_expansion_and_limit() {
    struct NoExpansionReplacer;

    impl Replacer for NoExpansionReplacer {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
        
        fn replace_append(&self, _cap: &Captures, _output: &mut String) {
            // No operation needed for no expansion
        }
    }

    let regex = Regex::new(r"xyz").unwrap(); // Example regex that doesn't match
    let text = "abcdef";
    let limit = 5; // Arbitrary limit

    let result = regex.replacen(text, limit, NoExpansionReplacer);
    
    assert_eq!(result, Cow::Borrowed(text));
}

