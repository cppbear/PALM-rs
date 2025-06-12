// Answer 0

#[test]
fn test_exec_dfa_reverse_suffix_no_match() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::sync::Arc;

    struct DummyExecReadOnly {
        suffixes: LiteralSearcher,
    }

    struct DummyExecNoSync<'c> {
        ro: &'c Arc<DummyExecReadOnly>,
        cache: &'c RefCell<ProgramCacheInner>,
    }

    let lcs = FreqyPacked {
        pat: vec![b'a'], // lcs.len() == 1
        char_len: 1,
        rare1: b'a',
        rare1i: 0,
        rare2: b'b',
        rare2i: 0,
    };

    let suffixes = LiteralSearcher {
        complete: true,
        lcp: lcs.clone(),
        lcs,
        matcher: Matcher::Empty,
    };

    let exec_read_only = Arc::new(DummyExecReadOnly {
        suffixes,
    });

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = DummyExecNoSync {
        ro: &exec_read_only,
        cache: &cache,
    };

    let text: &[u8] = b""; // end == text.len() = 0
    let original_start = 0;

    let result = exec.exec_dfa_reverse_suffix(text, original_start);

    match result {
        Some(dfa::Result::NoMatch(len)) => assert_eq!(len, text.len()),
        _ => panic!("Expected NoMatch but got a different result."),
    }
}

