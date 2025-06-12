// Answer 0

#[test]
fn test_find_at_with_match_type_dfa_many_start_0_text_length_1() {
    let text = b"a";
    let start = 0;
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly { 
            res: vec![String::from("a")], 
            nfa: Program::new(), 
            dfa: Program::new(), 
            dfa_reverse: Program::new(), 
            suffixes: LiteralSearcher::new(), 
            match_type: MatchType::DfaMany 
        }), 
        cache: &RefCell::new(ProgramCacheInner::new()) 
    };
    exec.find_at(text, start);
}

#[test]
fn test_find_at_with_match_type_dfa_many_start_0_text_length_8192() {
    let text = b"ab".repeat(4096);
    let start = 0;
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly { 
            res: vec![String::from("ab")], 
            nfa: Program::new(), 
            dfa: Program::new(), 
            dfa_reverse: Program::new(), 
            suffixes: LiteralSearcher::new(), 
            match_type: MatchType::DfaMany 
        }), 
        cache: &RefCell::new(ProgramCacheInner::new()) 
    };
    exec.find_at(text, start);
}

#[test]
fn test_find_at_with_match_type_dfa_many_start_8192_text_length_1() {
    let text = b"a";
    let start = 8192;
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly { 
            res: vec![String::from("a")], 
            nfa: Program::new(), 
            dfa: Program::new(), 
            dfa_reverse: Program::new(), 
            suffixes: LiteralSearcher::new(), 
            match_type: MatchType::DfaMany 
        }), 
        cache: &RefCell::new(ProgramCacheInner::new()) 
    };
    exec.find_at(text, start);
}

#[test]
fn test_find_at_with_match_type_dfa_many_start_8192_text_length_1048576() {
    let text = b"abc".repeat(349525); // 1048575 bytes
    let start = 8192;
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly { 
            res: vec![String::from("abc")], 
            nfa: Program::new(), 
            dfa: Program::new(), 
            dfa_reverse: Program::new(), 
            suffixes: LiteralSearcher::new(), 
            match_type: MatchType::DfaMany 
        }), 
        cache: &RefCell::new(ProgramCacheInner::new()) 
    };
    exec.find_at(text, start);
}

#[test]
#[should_panic]
fn test_find_at_with_match_type_dfa_many_matching_unreachable() {
    let text = b"";
    let start = 0;
    let exec = ExecNoSync { 
        ro: &Arc::new(ExecReadOnly { 
            res: vec![String::from("")], 
            nfa: Program::new(), 
            dfa: Program::new(), 
            dfa_reverse: Program::new(), 
            suffixes: LiteralSearcher::new(), 
            match_type: MatchType::DfaMany 
        }), 
        cache: &RefCell::new(ProgramCacheInner::new()) 
    };
    exec.find_at(text, start);
}

