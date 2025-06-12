// Answer 0

#[test]
fn test_is_anchor_end_match_false_due_to_lcs_not_suffix() {
    let text: Vec<u8> = vec![b'a'; 1 << 20 + 1]; // text.len() = 1048577
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program { is_anchored_end: true },
        dfa: Program {},
        dfa_reverse: Program {},
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType {},
    });

    let lcs = FreqyPacked {
        pat: vec![b'b'], // lcs.len() = 1
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'c',
        rare2i: 1,
    };
    
    let suffixes = LiteralSearcher {
        complete: false,
        lcp: lcs.clone(),
        lcs,
        matcher: Matcher::Empty,
    };
    
    let exec = ExecNoSync {
        ro: &ro,
        cache: &RefCell::new(ProgramCacheInner {
            cache: HashMap::new(),
        }),
    };

    exec.ro.suffixes = suffixes;

    let result = exec.is_anchor_end_match(&text);
}

