// Answer 0

#[test]
fn test_is_anchor_end_match_with_no_lcs() {
    struct Ro {
        nfa: Nfa,
        suffixes: Suffixes,
    }

    struct Nfa {
        is_anchored_end: bool,
    }

    struct Suffixes;

    impl Suffixes {
        fn lcs(&self) -> Vec<u8> {
            vec![] // Simulating lcs with len() == 0
        }

        fn is_suffix(&self, _text: &[u8]) -> bool {
            false // This won't be called as lcs length is zero
        }
    }

    struct Regex {
        ro: Ro,
    }

    impl Regex {
        fn is_anchor_end_match(&self, text: &[u8]) -> bool {
            if text.len() > (1 << 20) && self.ro.nfa.is_anchored_end {
                let lcs = self.ro.suffixes.lcs();
                if lcs.len() >= 1 && !lcs.is_suffix(text) {
                    return false;
                }
            }
            true
        }
    }

    let regex = Regex {
        ro: Ro {
            nfa: Nfa {
                is_anchored_end: true,
            },
            suffixes: Suffixes,
        },
    };

    let large_text = vec![b'a'; (1 << 20) + 1]; // Large text of size > 1MB
    assert_eq!(regex.is_anchor_end_match(&large_text), true);
}

