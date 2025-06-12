// Answer 0

#[test]
fn test_replacen_no_matches_with_limit_zero() {
    use std::borrow::Cow;

    struct ConstantReplacer {
        replacement: String,
    }

    impl ConstantReplacer {
        pub fn no_expansion(&self) -> Option<&String> {
            Some(&self.replacement)
        }
    }

    let regex = regex::Regex::new("abc").unwrap();
    
    let text = "def";
    let limit = 0;
    let rep = ConstantReplacer {
        replacement: "xyz".to_string(),
    };

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Owned("def".to_string()));
}

#[test]
fn test_replacen_no_matches_with_limit_one() {
    use std::borrow::Cow;

    struct ConstantReplacer {
        replacement: String,
    }

    impl ConstantReplacer {
        pub fn no_expansion(&self) -> Option<&String> {
            Some(&self.replacement)
        }
    }

    let regex = regex::Regex::new("abc").unwrap();
    
    let text = "def";
    let limit = 1;
    let rep = ConstantReplacer {
        replacement: "xyz".to_string(),
    };

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Owned("def".to_string()));
}

#[test]
fn test_replacen_replaces_one_match_with_limit_one() {
    use std::borrow::Cow;

    struct ConstantReplacer {
        replacement: String,
    }

    impl ConstantReplacer {
        pub fn no_expansion(&self) -> Option<&String> {
            Some(&self.replacement)
        }
    }

    let regex = regex::Regex::new("def").unwrap();
    
    let text = "defxyz";
    let limit = 1;
    let rep = ConstantReplacer {
        replacement: "abc".to_string(),
    };

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Owned("abcxyz".to_string()));
}

#[test]
fn test_replacen_replaces_multiple_matches_with_limit_two() {
    use std::borrow::Cow;

    struct ConstantReplacer {
        replacement: String,
    }

    impl ConstantReplacer {
        pub fn no_expansion(&self) -> Option<&String> {
            Some(&self.replacement)
        }
    }

    let regex = regex::Regex::new("abc").unwrap();

    let text = "abcabcabc";
    let limit = 2;
    let rep = ConstantReplacer {
        replacement: "xyz".to_string(),
    };

    let result = regex.replacen(text, limit, rep);
    assert_eq!(result, Cow::Owned("xyzxyzabc".to_string()));
}

