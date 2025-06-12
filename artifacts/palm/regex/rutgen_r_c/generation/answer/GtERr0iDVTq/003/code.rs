// Answer 0

#[test]
fn test_is_anchor_end_match_true_case() {
    use std::sync::Arc;
    use std::cell::RefCell;

    struct MockNfa {
        is_anchored_end: bool,
    }

    struct MockExecReadOnly {
        nfa: MockNfa,
        suffixes: LiteralSearcher,
    }

    struct MockExecNoSync<'c> {
        ro: &'c Arc<MockExecReadOnly>,
    }

    impl LiteralSearcher {
        fn new_empty() -> Self {
            Self::empty()
        }
    }

    let text = vec![b'a'; (1 << 20) + 1]; // Length > 1MB
    let suffixes = LiteralSearcher::new_empty(); // lcs.len() == 0
    let nfa = MockNfa { is_anchored_end: true };
    let ro = MockExecReadOnly { nfa, suffixes };

    let exec = MockExecNoSync { ro: &Arc::new(ro) };

    assert_eq!(exec.is_anchor_end_match(&text), true);
}

