// Answer 0

#[test]
fn test_only_utf8_enabled() {
    struct Regex {
        only_utf8: bool,
    }

    impl Regex {
        pub fn only_utf8(mut self, yes: bool) -> Self {
            self.only_utf8 = yes;
            self
        }
    }

    let regex = Regex { only_utf8: false };
    let updated_regex = regex.only_utf8(true);
    assert!(updated_regex.only_utf8);
}

#[test]
fn test_only_utf8_disabled() {
    struct Regex {
        only_utf8: bool,
    }

    impl Regex {
        pub fn only_utf8(mut self, yes: bool) -> Self {
            self.only_utf8 = yes;
            self
        }
    }

    let regex = Regex { only_utf8: true };
    let updated_regex = regex.only_utf8(false);
    assert!(!updated_regex.only_utf8);
}

