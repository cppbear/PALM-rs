// Answer 0

#[test]
fn test_step_case1() {
    let mut cache = Cache { jobs: vec![], visited: vec![0; 8] };
    let prog = Program {
        insts: vec![Inst::Ranges(InstRanges { goto: 1, ranges: vec![('a', 'b')] })],
        matches: vec![0],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let input_at = InputAt { pos: 0, c: Char('x' as u32), byte: None, len: 1 };
    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];

    let mut bounded = Bounded { prog: &prog, input: DummyInput {}, matches: &mut matches, slots: &mut slots, m: &mut cache };

    bounded.step(0, input_at);
}

#[test]
fn test_step_case2() {
    let mut cache = Cache { jobs: vec![], visited: vec![0; 8] };
    let prog = Program {
        insts: vec![Inst::Ranges(InstRanges { goto: 1, ranges: vec![('c', 'd')] })],
        matches: vec![0],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let input_at = InputAt { pos: 5, c: Char('e' as u32), byte: None, len: 1 };
    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];

    let mut bounded = Bounded { prog: &prog, input: DummyInput {}, matches: &mut matches, slots: &mut slots, m: &mut cache };

    bounded.step(0, input_at);
}

#[test]
fn test_step_case3() {
    let mut cache = Cache { jobs: vec![], visited: vec![0; 8] };
    let prog = Program {
        insts: vec![Inst::Ranges(InstRanges { goto: 1, ranges: vec![('a', 'c')] })],
        matches: vec![0],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let input_at = InputAt { pos: 0, c: Char('d' as u32), byte: None, len: 1 };
    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];

    let mut bounded = Bounded { prog: &prog, input: DummyInput {}, matches: &mut matches, slots: &mut slots, m: &mut cache };

    bounded.step(0, input_at);
}

