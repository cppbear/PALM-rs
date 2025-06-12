// Answer 0

#[test]
fn test_choose_match_type_nfa_empty() {
    let nfa = Program { insts: Vec::new(), matches: Vec::new(), captures: Vec::new(), capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: Vec::new(), only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::empty(), dfa_size_limit: 0 };
    let suffixes = LiteralSearcher::empty();
    let exec = ExecReadOnly { res: vec!["a".to_string()], nfa, dfa: nfa.clone(), dfa_reverse: nfa.clone(), suffixes, match_type: MatchType::Nothing };
    
    assert_eq!(exec.choose_match_type(None), MatchType::Nothing);
}

#[test]
fn test_choose_match_type_single_prefixes_complete() {
    let nfa = Program { insts: Vec::new(), matches: Vec::new(), captures: Vec::new(), capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: Vec::new(), only_utf8: false, is_bytes: false, is_dfa: false, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::prefixes(Literals::empty()), dfa_size_limit: 0 };
    let suffixes = LiteralSearcher::empty();
    let exec = ExecReadOnly { res: vec!["a".to_string()], nfa, dfa: Program::default(), dfa_reverse: Program::default(), suffixes, match_type: MatchType::Nothing };
    
    assert_eq!(exec.choose_match_type(None), MatchType::Literal(MatchLiteralType::Unanchored));
}

#[test]
fn test_choose_match_type_dfa_exec() {
    let nfa = Program { insts: vec![Inst::Match(0)], matches: vec![InstPtr::default()], captures: Vec::new(), capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: Vec::new(), only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::empty(), dfa_size_limit: 0 };
    let dfa = nfa.clone();
    let suffixes = LiteralSearcher::empty();
    let exec = ExecReadOnly { res: vec!["a".to_string()], nfa, dfa, dfa_reverse: Program::default(), suffixes, match_type: MatchType::Nothing };
    
    assert_eq!(exec.choose_match_type(None), MatchType::Dfa);
}

#[test]
fn test_choose_match_type_multiple_res() {
    let nfa = Program { insts: vec![Inst::Match(0)], matches: vec![InstPtr::default()], captures: Vec::new(), capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: Vec::new(), only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: false, has_unicode_word_boundary: false, prefixes: LiteralSearcher::empty(), dfa_size_limit: 0 };
    let dfa = nfa.clone();
    let suffixes = LiteralSearcher::empty();
    let exec = ExecReadOnly { res: vec!["a".to_string(), "b".to_string()], nfa, dfa, dfa_reverse: Program::default(), suffixes, match_type: MatchType::Nothing };
    
    assert_eq!(exec.choose_match_type(None), MatchType::DfaMany);
}

#[test]
fn test_choose_match_type_reverse_anchored() {
    let nfa = Program { insts: vec![Inst::Match(0)], matches: vec![InstPtr::default()], captures: Vec::new(), capture_name_idx: Arc::new(HashMap::new()), start: InstPtr::default(), byte_classes: Vec::new(), only_utf8: false, is_bytes: false, is_dfa: true, is_reverse: false, is_anchored_start: false, is_anchored_end: true, has_unicode_word_boundary: false, prefixes: LiteralSearcher::empty(), dfa_size_limit: 0 };
    let dfa = nfa.clone();
    let suffixes = LiteralSearcher::empty();
    let exec = ExecReadOnly { res: vec!["a".to_string()], nfa, dfa, dfa_reverse: Program::default(), suffixes, match_type: MatchType::Nothing };
    
    assert_eq!(exec.choose_match_type(None), MatchType::DfaAnchoredReverse);
}

