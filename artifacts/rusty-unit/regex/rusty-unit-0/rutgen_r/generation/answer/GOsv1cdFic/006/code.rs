// Answer 0

#[test]
fn test_forward_many_success_case() {
    struct DummyProgram {
        matches: Vec<usize>,
    }
    
    struct DummyProgramCache {
        pub dfa: DummyDfaCache,
    }
    
    struct DummyDfaCache {
        pub qcur: usize,
        pub qnext: usize,
        pub inner: usize,
    }

    let prog = DummyProgram { matches: vec![0] };
    let cache = DummyProgramCache { dfa: DummyDfaCache { qcur: 1, qnext: 2, inner: 3 } };
    let mut matches = vec![false];
    let text = b"some text to match";
    let at = 0;

    let result = forward_many(&prog, &cache, &mut matches, text, at);
    
    assert!(result.is_ok());
    assert!(matches[0]); // Assuming the FSM logic correctly matches the input.
}

#[test]
fn test_forward_many_no_match() {
    struct DummyProgram {
        matches: Vec<usize>,
    }

    struct DummyProgramCache {
        pub dfa: DummyDfaCache,
    }

    struct DummyDfaCache {
        pub qcur: usize,
        pub qnext: usize,
        pub inner: usize,
    }

    let prog = DummyProgram { matches: vec![0] };
    let cache = DummyProgramCache { dfa: DummyDfaCache { qcur: 1, qnext: 2, inner: 3 } };
    let mut matches = vec![false];
    let text = b"no matching text";
    let at = 0;

    let result = forward_many(&prog, &cache, &mut matches, text, at);
    
    assert!(result.is_err()); // Expecting no match to be an error result.
    assert!(!matches[0]); // No matches should be recorded.
}

#[test]
#[should_panic]
fn test_forward_many_panic_on_len_mismatch() {
    struct DummyProgram {
        matches: Vec<usize>,
    }

    struct DummyProgramCache {
        pub dfa: DummyDfaCache,
    }

    struct DummyDfaCache {
        pub qcur: usize,
        pub qnext: usize,
        pub inner: usize,
    }

    let prog = DummyProgram { matches: vec![0, 1] }; // Two matches
    let cache = DummyProgramCache { dfa: DummyDfaCache { qcur: 1, qnext: 2, inner: 3 } };
    let mut matches = vec![false]; // One match    
    let text = b"some text to match";
    let at = 0;

    // This should panic due to the length mismatch
    let _ = forward_many(&prog, &cache, &mut matches, text, at);
}

