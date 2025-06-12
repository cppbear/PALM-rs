// Answer 0

#[test]
fn test_step_success_case() {
    let prog = Program {
        insts: vec![Inst::Char(InstChar {
            goto: 1,
            c: 'a',
        })],
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
        dfa_size_limit: 0,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];
    let mut cache = Cache {
        jobs: vec![],
        visited: vec![0; 4],
    };

    let input = Bounded {
        prog: &prog,
        input: MyInput::new("a"),
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let at = InputAt {
        pos: 0,
        c: Char(b'a' as u32),
        byte: Some(b'a'),
        len: 1,
    };

    input.step(0, at);
}

#[test]
fn test_step_empty_match_case() {
    let prog = Program {
        insts: vec![Inst::Char(InstChar {
            goto: 1,
            c: 'a',
        })],
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
        dfa_size_limit: 0,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];
    let mut cache = Cache {
        jobs: vec![],
        visited: vec![0; 4],
    };

    let input = Bounded {
        prog: &prog,
        input: MyInput::new("b"),
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let at = InputAt {
        pos: 0,
        c: Char(b'b' as u32),
        byte: Some(b'b'),
        len: 1,
    };

    input.step(0, at);
}

#[test]
#[should_panic]
fn test_step_visited_case() {
    let prog = Program {
        insts: vec![Inst::Char(InstChar {
            goto: 1,
            c: 'a',
        })],
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
        dfa_size_limit: 0,
    };

    let mut matches = vec![false; 1];
    let mut slots = vec![None; 1];
    let mut cache = Cache {
        jobs: vec![],
        visited: vec![1; 4],
    };

    let input = Bounded {
        prog: &prog,
        input: MyInput::new("a"),
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let at = InputAt {
        pos: 0,
        c: Char(b'a' as u32),
        byte: Some(b'a'),
        len: 1,
    };

    input.step(0, at);
}

