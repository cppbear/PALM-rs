// Answer 0

#[test]
fn test_replacen_no_expansion_limit_zero_no_matches() {
    struct NoExpansion;

    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("nonexistent").unwrap();
    let text = "text with no matches";
    let limit = 0;
    let result = regex.replacen(text, limit, NoExpansion);
    assert_eq!(result, Cow::Owned("text with no matches".to_string()));
}

#[test]
fn test_replacen_no_expansion_limit_non_zero_with_matches() {
    struct NoExpansion;

    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replaced")
        }
    }

    let regex = Regex::new("replaceable").unwrap();
    let text = "replaceable text replaceable";
    let limit = 1;
    let result = regex.replacen(text, limit, NoExpansion);
    assert_eq!(result, Cow::Owned("replaced text replaceable".to_string()));
}

#[test]
fn test_replacen_capture_group_no_expansion_limit_zero_with_matches() {
    struct CaptureGroupReplacer;

    impl Replacer for CaptureGroupReplacer {
        fn no_expansion(&self) -> Option<&str> {
            None
        }
        
        fn replace_append(&self, cap: &Captures, new: &mut String) {
            new.push_str("GROUP ");
            new.push_str(&cap[0]);
        }
    }

    let regex = Regex::new(r"(\w+)").unwrap();
    let text = "word1 word2 word3";
    let limit = 0;
    let result = regex.replacen(text, limit, CaptureGroupReplacer);
    assert_eq!(result, Cow::Owned("GROUP word1 GROUP word2 GROUP word3".to_string()));
}

#[test]
#[should_panic]
fn test_replacen_capture_group_out_of_bounds() {
    struct CaptureGroupReplacer;

    impl Replacer for CaptureGroupReplacer {
        fn no_expansion(&self) -> Option<&str> {
            None
        }

        fn replace_append(&self, cap: &Captures, new: &mut String) {
            new.push_str(&cap[1]); // This will panic if there is no 1st capture
        }
    }

    let regex = Regex::new(r"(\w+)").unwrap();
    let text = "word";
    let limit = 1;
    let _ = regex.replacen(text, limit, CaptureGroupReplacer);
}

#[test]
#[should_panic]
fn test_replacen_last_match_out_of_bounds() {
    struct NoExpansion;

    impl Replacer for NoExpansion {
        fn no_expansion(&self) -> Option<&str> {
            Some("replacement")
        }
    }

    let regex = Regex::new("nonexistent").unwrap();
    let text = "some text";
    let limit = 0;
    let _ = regex.replacen(text, limit, NoExpansion); // limit 0, but no matches will cause panic
}

