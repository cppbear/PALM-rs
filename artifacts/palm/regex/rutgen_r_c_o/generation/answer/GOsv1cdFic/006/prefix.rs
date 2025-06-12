// Answer 0

#[test]
fn test_forward_many_case_1() {
    let insts = vec![Inst::Match(0)];
    let matches = vec![false];
    let prog = Program {
        insts,
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Default::default(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: Default::default(),
        dfa_size_limit: 1,
    };
    let mut cache = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: Default::default(),
        dfa_reverse: Default::default(),
    };
    let text = b"match";
    let at = 0;
    
    let result = Fsm::forward_many(&prog, &mut cache, &mut [matches[0]], text, at);
}

#[test]
fn test_forward_many_case_2() {
    let insts = vec![Inst::Match(1)];
    let matches = vec![false];
    let prog = Program {
        insts,
        matches: vec![1],
        captures: vec![],
        capture_name_idx: Default::default(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: Default::default(),
        dfa_size_limit: 1,
    };
    let mut cache = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: Default::default(),
        dfa_reverse: Default::default(),
    };
    let text = b"match";
    let at = 0;
    
    let result = Fsm::forward_many(&prog, &mut cache, &mut [matches[0]], text, at);
}

#[test]
fn test_forward_many_case_edge() {
    let insts = vec![Inst::Match(0)];
    let matches = vec![false];
    let prog = Program {
        insts,
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Default::default(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: Default::default(),
        dfa_size_limit: 1,
    };
    let mut cache = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: Default::default(),
        dfa_reverse: Default::default(),
    };
    let text = b"not a match";
    let at = 0;

    let result = Fsm::forward_many(&prog, &mut cache, &mut [matches[0]], text, at);
}

