// Answer 0

#[test]
fn test_is_anchor_end_match_false() {
    struct NFA {
        is_anchored_end: bool,
    }

    struct Suffixes {
        lcs: Vec<u8>,
    }

    struct RO {
        nfa: NFA,
        suffixes: Suffixes,
    }

    struct Matcher {
        ro: RO,
    }

    impl Suffixes {
        fn lcs(&self) -> &Vec<u8> {
            &self.lcs
        }
    }

    impl Suffixes {
        fn is_suffix(&self, text: &[u8]) -> bool {
            text.ends_with(&self.lcs)
        }
    }

    let text: Vec<u8> = vec![b'a'; (1 << 20) + 1]; // Just over 1MB
    let lcs = vec![b'a']; // lcs len == 1

    let matcher = Matcher {
        ro: RO {
            nfa: NFA {
                is_anchored_end: true,
            },
            suffixes: Suffixes {
                lcs,
            },
        },
    };

    assert_eq!(matcher.is_anchor_end_match(&text), false);
}

