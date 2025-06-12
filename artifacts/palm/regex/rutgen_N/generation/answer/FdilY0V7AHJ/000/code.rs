// Answer 0

#[test]
fn test_only_utf8_enabled() {
    struct Compiled {
        only_utf8: bool,
    }

    struct Regex {
        compiled: Compiled,
    }

    impl Regex {
        fn only_utf8(mut self, yes: bool) -> Self {
            self.compiled.only_utf8 = yes;
            self
        }
    }

    let regex = Regex { compiled: Compiled { only_utf8: false } };
    let regex_after = regex.only_utf8(true);
    assert!(regex_after.compiled.only_utf8);
}

#[test]
fn test_only_utf8_disabled() {
    struct Compiled {
        only_utf8: bool,
    }

    struct Regex {
        compiled: Compiled,
    }

    impl Regex {
        fn only_utf8(mut self, yes: bool) -> Self {
            self.compiled.only_utf8 = yes;
            self
        }
    }

    let regex = Regex { compiled: Compiled { only_utf8: true } };
    let regex_after = regex.only_utf8(false);
    assert!(!regex_after.compiled.only_utf8);
}

