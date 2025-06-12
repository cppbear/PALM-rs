// Answer 0

#[test]
fn test_forward_many_matches_length_mismatch() {
    struct DummyCache {
        dfa: ProgramCacheInner,
    }

    struct DummyProgram {
        matches: Vec<InstPtr>,
    }

    let mut matches = vec![false; 1]; // Length 1 for the test
    let prog = DummyProgram {
        matches: vec![0, 1], // Length 2 to trigger the panic condition
    };

    let cache = DummyCache {
        dfa: ProgramCacheInner {
            pikevm: pikevm::Cache::default(),
            backtrack: backtrack::Cache::default(),
            dfa: dfa::Cache::default(),
            dfa_reverse: dfa::Cache::default(),
        },
    };

    let text = b"test input";
    let at = 0;

    let result = std::panic::catch_unwind(|| {
        Fsm::forward_many(&prog, &cache, &mut matches, text, at)
    });

    assert!(result.is_err()); // Expecting a panic
}

