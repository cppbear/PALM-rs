// Answer 0

#[test]
fn test_reverse_set_to_true() {
    struct Compiled {
        is_reverse: bool,
    }

    struct Regex {
        compiled: Compiled,
    }

    impl Regex {
        pub fn reverse(mut self, yes: bool) -> Self {
            self.compiled.is_reverse = yes;
            self
        }
    }

    let regex = Regex {
        compiled: Compiled { is_reverse: false },
    };

    let reversed_regex = regex.reverse(true);
    assert!(reversed_regex.compiled.is_reverse);
}

#[test]
fn test_reverse_set_to_false() {
    struct Compiled {
        is_reverse: bool,
    }

    struct Regex {
        compiled: Compiled,
    }

    impl Regex {
        pub fn reverse(mut self, yes: bool) -> Self {
            self.compiled.is_reverse = yes;
            self
        }
    }

    let regex = Regex {
        compiled: Compiled { is_reverse: true },
    };

    let reversed_regex = regex.reverse(false);
    assert!(!reversed_regex.compiled.is_reverse);
}

