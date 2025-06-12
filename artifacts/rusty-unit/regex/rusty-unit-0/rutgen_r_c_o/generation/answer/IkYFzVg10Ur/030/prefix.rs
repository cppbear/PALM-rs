// Answer 0

#[test]
fn test_choose_match_type_case_1() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use syntax::hir::Inst;
    use dfa::can_exec;
    
    let dfa_program = Program {
        insts: vec![Inst::EmptyLook(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1,
    };

    let nfa_program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["a".to_string(), "b".to_string()],
        nfa: nfa_program,
        dfa: dfa_program,
        dfa_reverse: dfa_program,
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::Dfa,
    };

    let _result = exec_read_only.choose_match_type(Some(MatchType::Nfa(0)));
} 

#[test]
fn test_choose_match_type_case_2() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use syntax::hir::Inst;
    use dfa::can_exec;

    let dfa_program = Program {
        insts: vec![Inst::EmptyLook(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1,
    };

    let nfa_program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["a".to_string(), "c".to_string()],
        nfa: nfa_program,
        dfa: dfa_program,
        dfa_reverse: dfa_program,
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::Dfa,
    };

    let _result = exec_read_only.choose_match_type(Some(MatchType::Nfa(1)));
} 

#[test]
fn test_choose_match_type_case_3() {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use syntax::hir::Inst;
    use dfa::can_exec;

    let dfa_program = Program {
        insts: vec![Inst::EmptyLook(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 1,
    };

    let nfa_program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let suffixes = LiteralSearcher::empty();

    let exec_read_only = ExecReadOnly {
        res: vec!["d".to_string(), "e".to_string()],
        nfa: nfa_program,
        dfa: dfa_program,
        dfa_reverse: dfa_program,
        suffixes,
        match_type: MatchType::Dfa,
    };

    let _result = exec_read_only.choose_match_type(Some(MatchType::Nfa(2)));
}

