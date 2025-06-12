// Answer 0

#[test]
fn test_uses_bytes_when_is_bytes_true() {
    struct Prog {
        is_bytes: bool,
        is_dfa: bool,
    }

    impl Prog {
        pub fn uses_bytes(&self) -> bool {
            self.is_bytes || self.is_dfa
        }
    }

    let prog = Prog { is_bytes: true, is_dfa: false };
    assert!(prog.uses_bytes());
}

#[test]
fn test_uses_bytes_when_is_dfa_true() {
    struct Prog {
        is_bytes: bool,
        is_dfa: bool,
    }

    impl Prog {
        pub fn uses_bytes(&self) -> bool {
            self.is_bytes || self.is_dfa
        }
    }

    let prog = Prog { is_bytes: false, is_dfa: true };
    assert!(prog.uses_bytes());
}

#[test]
fn test_uses_bytes_when_both_true() {
    struct Prog {
        is_bytes: bool,
        is_dfa: bool,
    }

    impl Prog {
        pub fn uses_bytes(&self) -> bool {
            self.is_bytes || self.is_dfa
        }
    }

    let prog = Prog { is_bytes: true, is_dfa: true };
    assert!(prog.uses_bytes());
}

#[test]
fn test_uses_bytes_when_both_false() {
    struct Prog {
        is_bytes: bool,
        is_dfa: bool,
    }

    impl Prog {
        pub fn uses_bytes(&self) -> bool {
            self.is_bytes || self.is_dfa
        }
    }

    let prog = Prog { is_bytes: false, is_dfa: false };
    assert!(!prog.uses_bytes());
}

