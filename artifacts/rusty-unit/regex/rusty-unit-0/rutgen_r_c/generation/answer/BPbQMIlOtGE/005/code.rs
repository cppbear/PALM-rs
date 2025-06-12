// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("REPLACED")
        }
        
        fn replace_append(&self, _cap: &Captures, _dst: &mut String) {
            // No action needed
        }
    }

    let regex = Regex::new("nonexistent").unwrap();
    let result = regex.replacen("No matches here", 5, TestReplacer);
    assert_eq!(result, Cow::Owned("No matches here".to_string()));
}

#[test]
fn test_replacen_with_no_expansion_with_matches() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("REPLACED")
        }
        
        fn replace_append(&self, _cap: &Captures, _dst: &mut String) {
            // No action needed
        }
    }

    let regex = Regex::new("a").unwrap();
    let result = regex.replacen("banana", 2, TestReplacer);
    assert_eq!(result, Cow::Owned("bREPLACEdREPLACEn".to_string()));
}

#[test]
fn test_replacen_with_limit_zero() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("REPLACE")
        }

        fn replace_append(&self, _cap: &Captures, _dst: &mut String) {
            // No action needed
        }
    }

    let regex = Regex::new("b").unwrap();
    let result = regex.replacen("banana", 0, TestReplacer);
    assert_eq!(result, Cow::Owned("REPLACEana".to_string()));
}

#[test]
fn test_replacen_with_limit_exceeding_matches() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("X")
        }

        fn replace_append(&self, _cap: &Captures, _dst: &mut String) {
            // No action needed
        }
    }

    let regex = Regex::new("a").unwrap();
    let result = regex.replacen("aaaa", 10, TestReplacer);
    assert_eq!(result, Cow::Owned("X".to_string()));
}

#[test]
fn test_replacen_handles_last_match_correctly() {
    struct TestReplacer;

    impl Replacer for TestReplacer {
        fn no_expansion(&self) -> Option<&'static str> {
            Some("X")
        }

        fn replace_append(&self, _cap: &Captures, _dst: &mut String) {
            // No action needed
        }
    }

    let regex = Regex::new("a").unwrap();
    let result = regex.replacen("abcabcabc", 2, TestReplacer);
    assert_eq!(result, Cow::Owned("XbcXbcabc".to_string()));
}

