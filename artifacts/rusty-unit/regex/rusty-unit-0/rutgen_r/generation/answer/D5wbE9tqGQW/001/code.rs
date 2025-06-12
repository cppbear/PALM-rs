// Answer 0

#[test]
fn test_needs_dotstar_true() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    impl Config {
        pub fn needs_dotstar(&self) -> bool {
            self.is_dfa && !self.is_reverse && !self.is_anchored_start
        }
    }

    let config = Config {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
    };

    assert_eq!(config.needs_dotstar(), true);
}

#[test]
fn test_needs_dotstar_false_is_reverse() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    impl Config {
        pub fn needs_dotstar(&self) -> bool {
            self.is_dfa && !self.is_reverse && !self.is_anchored_start
        }
    }

    let config = Config {
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
    };

    assert_eq!(config.needs_dotstar(), false);
}

#[test]
fn test_needs_dotstar_false_is_anchored_start() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    impl Config {
        pub fn needs_dotstar(&self) -> bool {
            self.is_dfa && !self.is_reverse && !self.is_anchored_start
        }
    }

    let config = Config {
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
    };

    assert_eq!(config.needs_dotstar(), false);
}

#[test]
fn test_needs_dotstar_false_not_dfa() {
    struct Config {
        is_dfa: bool,
        is_reverse: bool,
        is_anchored_start: bool,
    }

    impl Config {
        pub fn needs_dotstar(&self) -> bool {
            self.is_dfa && !self.is_reverse && !self.is_anchored_start
        }
    }

    let config = Config {
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
    };

    assert_eq!(config.needs_dotstar(), false);
}

