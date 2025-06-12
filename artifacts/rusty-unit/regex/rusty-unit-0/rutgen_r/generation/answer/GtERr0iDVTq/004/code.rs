// Answer 0

#[test]
fn test_is_anchor_end_match_with_large_text_and_non_anchored() {
    struct MockNFA {
        is_anchored_end: bool,
    }

    struct MockSuffixes;

    impl MockSuffixes {
        fn lcs(&self) -> Vec<u8> {
            vec![b'a'] // Dummy value for LCS
        }
    }

    struct Regex {
        ro: RegexRO,
    }

    struct RegexRO {
        nfa: MockNFA,
        suffixes: MockSuffixes,
    }

    let text = vec![b'a'; 1 << 21]; // Create a byte vector larger than 1MB
    let regex = Regex {
        ro: RegexRO {
            nfa: MockNFA { is_anchored_end: false },
            suffixes: MockSuffixes,
        },
    };

    assert_eq!(regex.is_anchor_end_match(&text), true);
}

