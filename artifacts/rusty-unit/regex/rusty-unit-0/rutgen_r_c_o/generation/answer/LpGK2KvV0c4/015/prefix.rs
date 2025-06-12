// Answer 0

#[test]
fn test_step_with_slot_equal_to_matches_len() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut matches = vec![false];
    let mut slots = vec![None; 1];
    let input = MyInput::new("sample input");
    let mut cache = Cache { inner: CacheInner::new(), qcur: SparseSet::new(), qnext: SparseSet::new() };
    let mut bounded = Bounded { prog: &prog, input, matches: &mut matches, slots: &mut slots, m: &mut cache };

    let at = InputAt { pos: 11, c: Char(0), byte: None, len: 11 };
    let ip = InstPtr(0);

    bounded.step(ip, at);
}

#[test]
fn test_step_with_hitting_match_without_updating_matches() {
    let prog = Program {
        insts: vec![Inst::Match(1)],
        matches: vec![true, false],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut matches = vec![true, false];
    let mut slots = vec![None; 2];
    let input = MyInput::new("sample input");
    let mut cache = Cache { inner: CacheInner::new(), qcur: SparseSet::new(), qnext: SparseSet::new() };
    let mut bounded = Bounded { prog: &prog, input, matches: &mut matches, slots: &mut slots, m: &mut cache };

    let at = InputAt { pos: 11, c: Char(0), byte: None, len: 11 };
    let ip = InstPtr(0);

    bounded.step(ip, at);
}

#[test]
fn test_step_with_empty_input_and_no_visits() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![true],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut matches = vec![false];
    let mut slots = vec![None];
    let input = MyInput::new("");
    let mut cache = Cache { inner: CacheInner::new(), qcur: SparseSet::new(), qnext: SparseSet::new() };
    let mut bounded = Bounded { prog: &prog, input, matches: &mut matches, slots: &mut slots, m: &mut cache };

    let at = InputAt { pos: 0, c: Char(0), byte: None, len: 0 };
    let ip = InstPtr(0);

    bounded.step(ip, at);
}

#[test]
fn test_step_with_different_positions_and_matches_len() {
    let prog = Program {
        insts: vec![Inst::Match(1)],
        matches: vec![false, false],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: InstPtr(0),
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut matches = vec![false, false];
    let mut slots = vec![None, None];
    let input = MyInput::new("sample input");
    let mut cache = Cache { inner: CacheInner::new(), qcur: SparseSet::new(), qnext: SparseSet::new() };
    let mut bounded = Bounded { prog: &prog, input, matches: &mut matches, slots: &mut slots, m: &mut cache };

    let at = InputAt { pos: 11, c: Char(0), byte: None, len: 11 };
    let ip = InstPtr(0);

    bounded.step(ip, at);
}

