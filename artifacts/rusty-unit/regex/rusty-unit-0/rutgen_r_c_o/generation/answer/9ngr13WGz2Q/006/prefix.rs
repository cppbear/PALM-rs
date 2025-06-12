// Answer 0

#[test]
fn test_replacen_empty_text_no_expansion() {
    struct MockReplacer {
        no_expansion: Option<&'static [u8]>,
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            self.no_expansion
        }
    }

    let regex = Regex::new("pattern").unwrap();
    let text = b"";
    let limit = 0;
    let rep = MockReplacer {
        no_expansion: Some(b"replacement"),
    };

    let _result = regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_empty_text_no_expansion_limit_zero() {
    struct MockReplacer {
        no_expansion: Option<&'static [u8]>,
    }

    impl MockReplacer {
        fn no_expansion(&self) -> Option<&'static [u8]> {
            self.no_expansion
        }
    }

    let regex = Regex::new("pattern").unwrap();
    let text = b"";
    let limit = 0;
    let rep = MockReplacer {
        no_expansion: Some(b"replacement"),
    };

    let _result = regex.replacen(text, limit, rep);
}

