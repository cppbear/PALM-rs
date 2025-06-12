// Answer 0

#[test]
fn test_is_anchor_end_match_below_size_limit() {
    struct Regex {
        ro: RegexOptions,
    }

    struct RegexOptions {
        nfa: Nfa,
        suffixes: Suffixes,
    }

    struct Nfa {
        is_anchored_end: bool,
    }

    struct Suffixes;

    impl Suffixes {
        fn lcs(&self) -> Vec<u8> {
            vec![b'a'] // Example longest common suffix
        }

        fn is_suffix(&self, text: &[u8]) -> bool {
            text.ends_with(&self.lcs())
        }
    }

    let regex = Regex {
        ro: RegexOptions {
            nfa: Nfa {
                is_anchored_end: true,
            },
            suffixes: Suffixes,
        },
    };

    let result = regex.is_anchor_end_match(b"small text");
    assert!(result);
}

#[test]
fn test_is_anchor_end_match_above_size_limit_with_no_suffix() {
    struct Regex {
        ro: RegexOptions,
    }

    struct RegexOptions {
        nfa: Nfa,
        suffixes: Suffixes,
    }

    struct Nfa {
        is_anchored_end: bool,
    }

    struct Suffixes;

    impl Suffixes {
        fn lcs(&self) -> Vec<u8> {
            vec![b'a'] // Example longest common suffix
        }

        fn is_suffix(&self, text: &[u8]) -> bool {
            false
        }
    }

    let regex = Regex {
        ro: RegexOptions {
            nfa: Nfa {
                is_anchored_end: true,
            },
            suffixes: Suffixes,
        },
    };

    let result = regex.is_anchor_end_match(&vec![0; (1 << 20) + 1]);
    assert!(!result);
}

#[test]
fn test_is_anchor_end_match_above_size_limit_with_suffix() {
    struct Regex {
        ro: RegexOptions,
    }

    struct RegexOptions {
        nfa: Nfa,
        suffixes: Suffixes,
    }

    struct Nfa {
        is_anchored_end: bool,
    }

    struct Suffixes;

    impl Suffixes {
        fn lcs(&self) -> Vec<u8> {
            vec![b'a'] // Example longest common suffix
        }

        fn is_suffix(&self, text: &[u8]) -> bool {
            text.ends_with(&self.lcs())
        }
    }

    let regex = Regex {
        ro: RegexOptions {
            nfa: Nfa {
                is_anchored_end: true,
            },
            suffixes: Suffixes,
        },
    };

    let result = regex.is_anchor_end_match(&[b'b'; (1 << 20) + 1]);
    assert!(result);
}

#[test]
fn test_is_anchor_end_match_size_limit_edge_case() {
    struct Regex {
        ro: RegexOptions,
    }

    struct RegexOptions {
        nfa: Nfa,
        suffixes: Suffixes,
    }

    struct Nfa {
        is_anchored_end: bool,
    }

    struct Suffixes;

    impl Suffixes {
        fn lcs(&self) -> Vec<u8> {
            vec![b'a']
        }

        fn is_suffix(&self, text: &[u8]) -> bool {
            text.ends_with(&self.lcs())
        }
    }

    let regex = Regex {
        ro: RegexOptions {
            nfa: Nfa {
                is_anchored_end: true,
            },
            suffixes: Suffixes,
        },
    };

    let result = regex.is_anchor_end_match(&vec![0; (1 << 20)]);
    assert!(result);
}

