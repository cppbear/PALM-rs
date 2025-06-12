// Answer 0

#[test]
fn test_replacen_no_expansion_zero_limit() {
    struct SimpleReplacer<'a>(&'a str);

    impl<'a> Replacer for SimpleReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }
        
        fn replace_append(&self, _cap: &Captures<'_>, _dst: &mut String) {
            // This won't be used since no captures are involved
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let result = regex.replacen("abc123def456", 0, SimpleReplacer("XY"));
    assert_eq!(result, Cow::Owned(String::from("abcXYdefXY")));
}

#[test]
fn test_replacen_no_expansion_limit_one() {
    struct SimpleReplacer<'a>(&'a str);

    impl<'a> Replacer for SimpleReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }

        fn replace_append(&self, _cap: &Captures<'_>, _dst: &mut String) {
            // This won't be used since no captures are involved
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let result = regex.replacen("abc123def456", 1, SimpleReplacer("XY"));
    assert_eq!(result, Cow::Owned(String::from("abcXYdef456")));
}

#[test]
fn test_replacen_no_expansion_limit_exceeding() {
    struct SimpleReplacer<'a>(&'a str);

    impl<'a> Replacer for SimpleReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }

        fn replace_append(&self, _cap: &Captures<'_>, _dst: &mut String) {
            // This won't be used since no captures are involved
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let result = regex.replacen("abc123def456", 5, SimpleReplacer("XY"));
    assert_eq!(result, Cow::Owned(String::from("abcXYdefXY")));
}

#[test]
fn test_replacen_no_matches() {
    struct SimpleReplacer<'a>(&'a str);

    impl<'a> Replacer for SimpleReplacer<'a> {
        fn no_expansion(&self) -> Option<&str> {
            Some(self.0)
        }

        fn replace_append(&self, _cap: &Captures<'_>, _dst: &mut String) {
            // This won't be used since no captures are involved
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let result = regex.replacen("abcdef", 0, SimpleReplacer("XY"));
    assert_eq!(result, Cow::Borrowed("abcdef"));
}

