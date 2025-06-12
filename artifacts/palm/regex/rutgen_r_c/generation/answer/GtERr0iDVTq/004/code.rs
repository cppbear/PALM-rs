// Answer 0

#[test]
fn test_is_anchor_end_match_on_large_text_with_non_anchored_end() {
    struct MockExecReadOnly {
        nfa: Nfa,
        suffixes: LiteralSearcher,
    }

    struct MockExecNoSync<'c> {
        ro: &'c MockExecReadOnly,
    }

    #[derive(Clone)]
    struct Nfa {
        is_anchored_end: bool,
    }

    let suffixes = LiteralSearcher::empty();
    let nfa = Nfa { is_anchored_end: false };
    let ro = MockExecReadOnly { nfa, suffixes };
    let exec = MockExecNoSync { ro: &ro };

    let large_text: Vec<u8> = vec![b'a'; (1 << 20) + 1]; // Just over 1MB

    let result = exec.is_anchor_end_match(&large_text);

    assert_eq!(result, true);
}

