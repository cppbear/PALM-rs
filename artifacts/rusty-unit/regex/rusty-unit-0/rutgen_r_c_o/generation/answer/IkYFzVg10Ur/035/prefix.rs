// Answer 0

#[test]
fn test_choose_match_type_hint_nfa_non_empty_inst_anchored_end() {
    use regex::{ExecReadOnly, MatchType, Program, Inst};
    use std::cell::RefCell;

    let program = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 128,
    };

    let exec_read_only = ExecReadOnly {
        res: vec!["regex".to_string()],
        nfa: program.clone(),
        dfa: program,
        dfa_reverse: program,
        suffixes: LiteralSearcher::empty(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    };

    let _result = exec_read_only.choose_match_type(Some(MatchType::Nfa(MatchNfaType::Auto)));
}

