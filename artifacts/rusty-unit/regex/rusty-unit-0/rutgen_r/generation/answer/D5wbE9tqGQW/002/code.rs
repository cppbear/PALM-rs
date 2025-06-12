// Answer 0

#[test]
fn test_needs_dotstar_true() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let config = Config {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
    };

    assert_eq!(needs_dotstar(&config), true);
}

#[test]
fn test_needs_dotstar_false_anchored_start_true() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let config = Config {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
    };

    assert_eq!(needs_dotstar(&config), false);
}

#[test]
fn test_needs_dotstar_false_reverse_true() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let config = Config {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
    };

    assert_eq!(needs_dotstar(&config), false);
}

#[test]
fn test_needs_dotstar_false_dfa_false() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    let config = Config {
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
    };

    assert_eq!(needs_dotstar(&config), false);
}

