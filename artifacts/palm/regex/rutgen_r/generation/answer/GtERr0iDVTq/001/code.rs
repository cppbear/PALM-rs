// Answer 0

#[test]
fn test_is_anchor_end_match_success() {
    struct NFA {
        is_anchored_end: bool,
    }

    struct RO {
        nfa: NFA,
        suffixes: LCS,
    }

    struct LCS {
        value: Vec<u8>,
    }

    impl LCS {
        fn lcs(&self) -> &Vec<u8> {
            &self.value
        }
        
        fn is_suffix(&self, text: &[u8]) -> bool {
            let text_len = text.len();
            let lcs_len = self.value.len();
            if lcs_len == 0 || text_len < lcs_len {
                return false;
            }
            &text[text_len - lcs_len..] == &self.value[..]
        }
    }

    struct Matcher {
        ro: RO,
    }

    impl Matcher {
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

    let nfa = NFA { is_anchored_end: true };
    let lcs_value = vec![b's', b'u', b'f', b'f', b'i', b'x']; // A known suffix
    let suffixes = LCS { value: lcs_value };
    let ro = RO { nfa, suffixes };
    
    let matcher = Matcher { ro };
    
    // Prepare text with size greater than 1MB with a known suffix
    let text = vec![b'a'; (1 << 20) + 1]; // 1MB + 1 byte
    text.extend_from_slice(b"suffix"); // Ensuring the text ends with the defined suffix

    assert!(matcher.is_anchor_end_match(&text));
}

