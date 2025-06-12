// Answer 0

#[test]
fn test_dfa_true() {
    struct Compiled {
        is_dfa: bool,
    }

    struct Machine {
        compiled: Compiled,
    }

    let mut machine = Machine {
        compiled: Compiled { is_dfa: false },
    };

    machine = machine.dfa(true);
    assert!(machine.compiled.is_dfa);
}

#[test]
fn test_dfa_false() {
    struct Compiled {
        is_dfa: bool,
    }

    struct Machine {
        compiled: Compiled,
    }

    let mut machine = Machine {
        compiled: Compiled { is_dfa: true },
    };

    machine = machine.dfa(false);
    assert!(!machine.compiled.is_dfa);
}

