// Answer 0

#[test]
fn test_only_utf8_enabled() {
    struct RegexExec {
        only_utf8: bool,
    }

    impl RegexExec {
        fn only_utf8(mut self, yes: bool) -> Self {
            self.only_utf8 = yes;
            self
        }
    }

    let mut exec = RegexExec { only_utf8: false };
    exec = exec.only_utf8(true);
    assert!(exec.only_utf8);
}

#[test]
fn test_only_utf8_disabled() {
    struct RegexExec {
        only_utf8: bool,
    }

    impl RegexExec {
        fn only_utf8(mut self, yes: bool) -> Self {
            self.only_utf8 = yes;
            self
        }
    }

    let mut exec = RegexExec { only_utf8: true };
    exec = exec.only_utf8(false);
    assert!(!exec.only_utf8);
}

