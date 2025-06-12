// Answer 0

#[test]
fn test_is_anchor_end_match_true() {
    struct MockExecReadOnly {
        is_anchored_end: bool,
        suffixes: LiteralSearcher,
    }

    struct MockExecNoSync<'c> {
        ro: &'c MockExecReadOnly,
    }

    let suffix_freqy_packed = FreqyPacked {
        pat: vec![b'a'],
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 1,
    };
    
    let mock_suffixes = LiteralSearcher {
        complete: true,
        lcp: suffix_freqy_packed.clone(),
        lcs: suffix_freqy_packed.clone(),
        matcher: Matcher::Empty,
    };

    let mock_read_only = MockExecReadOnly {
        is_anchored_end: true,
        suffixes: mock_suffixes,
    };
    
    let exec_no_sync = MockExecNoSync {
        ro: &mock_read_only,
    };

    let text: Vec<u8> = vec![b'a'; (1 << 20) + 1]; // Length greater than 1MB

    assert_eq!(exec_no_sync.is_anchor_end_match(&text), true);
}

