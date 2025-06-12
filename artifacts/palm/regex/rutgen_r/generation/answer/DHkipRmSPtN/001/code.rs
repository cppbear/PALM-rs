// Answer 0

#[test]
fn test_program_cache_inner_new() {
    struct MockNfa;
    struct MockDfa;

    struct ExecReadOnly {
        nfa: MockNfa,
        dfa: MockDfa,
        dfa_reverse: MockDfa,
    }

    impl ExecReadOnly {
        fn new() -> Self {
            Self {
                nfa: MockNfa,
                dfa: MockDfa,
                dfa_reverse: MockDfa,
            }
        }
    }

    struct ProgramCacheInner {
        pikevm: pikevm::Cache<MockNfa>,
        backtrack: backtrack::Cache<MockNfa>,
        dfa: dfa::Cache<MockDfa>,
        dfa_reverse: dfa::Cache<MockDfa>,
    }

    fn new(ro: &ExecReadOnly) -> ProgramCacheInner {
        ProgramCacheInner {
            pikevm: pikevm::Cache::new(&ro.nfa),
            backtrack: backtrack::Cache::new(&ro.nfa),
            dfa: dfa::Cache::new(&ro.dfa),
            dfa_reverse: dfa::Cache::new(&ro.dfa_reverse),
        }
    }

    let ro = ExecReadOnly::new();
    let cache = new(&ro);

    assert!(cache.pikevm.is_some());
    assert!(cache.backtrack.is_some());
    assert!(cache.dfa.is_some());
    assert!(cache.dfa_reverse.is_some());
}

