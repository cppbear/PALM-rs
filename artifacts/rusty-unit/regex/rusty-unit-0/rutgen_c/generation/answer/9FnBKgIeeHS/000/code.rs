// Answer 0

#[test]
fn test_exec_pikevm_byte_input_match() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let prog = Program::new();
    let ro = ExecReadOnly {
        res: vec![String::from("test")],
        nfa: prog.clone(),
        dfa: prog.clone(),
        dfa_reverse: prog.clone(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let exec = ExecNoSync {
        ro: Arc::new(ro),
        cache: &cache,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![];
    let text = b"test";
    let start = 0;

    assert!(exec.exec_pikevm(&mut matches, &mut slots, false, text, start));
}

#[test]
fn test_exec_pikevm_char_input_match() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let mut prog = Program::new();
    prog.is_bytes = false;

    let ro = ExecReadOnly {
        res: vec![String::from("test")],
        nfa: prog.clone(),
        dfa: prog.clone(),
        dfa_reverse: prog.clone(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });
    
    let exec = ExecNoSync {
        ro: Arc::new(ro),
        cache: &cache,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![];
    let text = b"test";
    let start = 0;

    assert!(exec.exec_pikevm(&mut matches, &mut slots, false, text, start));
}

#[test]
fn test_exec_pikevm_no_match() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let prog = Program::new();

    let ro = ExecReadOnly {
        res: vec![String::from("test")],
        nfa: prog.clone(),
        dfa: prog.clone(),
        dfa_reverse: prog.clone(),
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::default(),
    };
    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let exec = ExecNoSync {
        ro: Arc::new(ro),
        cache: &cache,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![];
    let text = b"mismatch";
    let start = 0;

    assert!(!exec.exec_pikevm(&mut matches, &mut slots, false, text, start));
}

