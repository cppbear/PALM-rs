// Answer 0

#[test]
fn test_find_at_literal_match() {
    use std::cell::RefCell;
    use std::sync::Arc;
    
    let match_type = MatchType::Literal(MatchLiteralType::Unanchored);
    let nfa = Program::new();
    let dfa = Program::new();
    let dfa_reverse = Program::new();
    let suffixes = LiteralSearcher::new();
    let ro = Arc::new(ExecReadOnly {
        res: vec!["0".to_string(), "1".to_string()],
        nfa,
        dfa,
        dfa_reverse,
        suffixes,
        match_type,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };
    
    let text = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    exec.find_at(&text, 0);
    exec.find_at(&text, 1);
    exec.find_at(&text, 5);
}

#[test]
fn test_find_at_literal_match_end() {
    use std::cell::RefCell;
    use std::sync::Arc;
    
    let match_type = MatchType::Literal(MatchLiteralType::AnchoredEnd);
    let nfa = Program::new();
    let dfa = Program::new();
    let dfa_reverse = Program::new();
    let suffixes = LiteralSearcher::new();
    let ro = Arc::new(ExecReadOnly {
        res: vec!["9".to_string()],
        nfa,
        dfa,
        dfa_reverse,
        suffixes,
        match_type,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };
    
    let text = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    exec.find_at(&text, 9);
}

#[test]
fn test_find_at_literal_match_start() {
    use std::cell::RefCell;
    use std::sync::Arc;
    
    let match_type = MatchType::Literal(MatchLiteralType::AnchoredStart);
    let nfa = Program::new();
    let dfa = Program::new();
    let dfa_reverse = Program::new();
    let suffixes = LiteralSearcher::new();
    let ro = Arc::new(ExecReadOnly {
        res: vec!["0".to_string()],
        nfa,
        dfa,
        dfa_reverse,
        suffixes,
        match_type,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };
    
    let text = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    exec.find_at(&text, 0);
}

#[test]
fn test_find_at_no_match() {
    use std::cell::RefCell;
    use std::sync::Arc;
    
    let match_type = MatchType::Literal(MatchLiteralType::AnchoredStart);
    let nfa = Program::new();
    let dfa = Program::new();
    let dfa_reverse = Program::new();
    let suffixes = LiteralSearcher::new();
    let ro = Arc::new(ExecReadOnly {
        res: vec!["99".to_string()],
        nfa,
        dfa,
        dfa_reverse,
        suffixes,
        match_type,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };
    
    let text = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    exec.find_at(&text, 0);
}

#[test]
fn test_find_at_empty_text() {
    use std::cell::RefCell;
    use std::sync::Arc;
    
    let match_type = MatchType::Literal(MatchLiteralType::AnchoredStart);
    let nfa = Program::new();
    let dfa = Program::new();
    let dfa_reverse = Program::new();
    let suffixes = LiteralSearcher::new();
    let ro = Arc::new(ExecReadOnly {
        res: vec!["0".to_string()],
        nfa,
        dfa,
        dfa_reverse,
        suffixes,
        match_type,
    });
    let cache = RefCell::new(ProgramCacheInner::new());
    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };
    
    let text: [u8; 0] = [];
    exec.find_at(&text, 0);    
}

