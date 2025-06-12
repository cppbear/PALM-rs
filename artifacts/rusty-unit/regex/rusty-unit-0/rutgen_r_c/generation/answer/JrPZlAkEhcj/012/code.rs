// Answer 0

#[test]
fn test_shortest_match_at_literal_success() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::cell::RefCell;
    
    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
        nfa: Program,
        match_type: MatchType,
    }

    #[derive(Debug)]
    struct Program {
        prefixes: LiteralSearcher,
        is_anchored_end: bool,
    }

    #[derive(Debug)]
    struct LiteralSearcher;

    impl LiteralSearcher {
        fn find(&self, text: &[u8]) -> Option<(usize, usize)> {
            // Simulate successful match
            Some((0, 5))
        }
    }
    
    let text: &[u8] = b"hello world";
    let program = Program {
        prefixes: LiteralSearcher,
        is_anchored_end: true,
    };
    
    let ro = ExecReadOnly {
        res: vec![],
        nfa: program,
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };
    
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &cache,
    };

    assert_eq!(exec.shortest_match_at(text, 0), Some(5));
}

#[test]
fn test_shortest_match_at_literal_no_match() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::cell::RefCell;
    
    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
        nfa: Program,
        match_type: MatchType,
    }

    #[derive(Debug)]
    struct Program {
        prefixes: LiteralSearcher,
        is_anchored_end: bool,
    }

    #[derive(Debug)]
    struct LiteralSearcher;

    impl LiteralSearcher {
        fn find(&self, text: &[u8]) -> Option<(usize, usize)> {
            // Simulate no match
            None
        }
    }
    
    let text: &[u8] = b"goodbye world";
    let program = Program {
        prefixes: LiteralSearcher,
        is_anchored_end: true,
    };
    
    let ro = ExecReadOnly {
        res: vec![],
        nfa: program,
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };
    
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &cache,
    };

    assert_eq!(exec.shortest_match_at(text, 0), None);
}

#[test]
#[should_panic]
fn test_shortest_match_at_literal_invalid_state() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::cell::RefCell;
    
    #[derive(Debug)]
    struct ExecReadOnly {
        res: Vec<String>,
        nfa: Program,
        match_type: MatchType,
    }

    #[derive(Debug)]
    struct Program {
        prefixes: LiteralSearcher,
        is_anchored_end: bool,
    }

    #[derive(Debug)]
    struct LiteralSearcher;

    impl LiteralSearcher {
        fn find(&self, text: &[u8]) -> Option<(usize, usize)> {
            // Simulate invalid behavior
            panic!("Invalid state during matching");
        }
    }
    
    let text: &[u8] = b"panic test";
    let program = Program {
        prefixes: LiteralSearcher,
        is_anchored_end: true,
    };
    
    let ro = ExecReadOnly {
        res: vec![],
        nfa: program,
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    };
    
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let exec = ExecNoSync {
        ro: &Arc::new(ro),
        cache: &cache,
    };

    exec.shortest_match_at(text, 0);
}

