// Answer 0

#[test]
fn test_step_with_valid_inputs() {
    let program = Program {
        insts: vec![Inst::Match(0)],
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

    let matches = &mut [false; 1];
    let slots = &mut [None; 1];
    let cache = Cache {
        jobs: vec![],
        visited: vec![0; MAX_SIZE_BYTES],
    };

    let input_at = InputAt {
        pos: 0,
        c: Char(0),
        byte: Some(0),
        len: 1,
    };

    let mut bounded = Bounded {
        prog: &program,
        input: DummyInput {},
        matches,
        slots,
        m: &mut cache,
    };

    bounded.step(0, input_at);
}

#[test]
fn test_step_with_different_inst() {
    let program = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
        matches: vec![false, false],
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

    let matches = &mut [false; 2];
    let slots = &mut [None; 2];
    let cache = Cache {
        jobs: vec![],
        visited: vec![0; MAX_SIZE_BYTES],
    };

    let input_at = InputAt {
        pos: 0,
        c: Char(1),
        byte: Some(1),
        len: 1,
    };

    let mut bounded = Bounded {
        prog: &program,
        input: DummyInput {},
        matches,
        slots,
        m: &mut cache,
    };

    bounded.step(0, input_at);
}

#[test]
fn test_step_with_edge_case() {
    let program = Program {
        insts: vec![Inst::Match(0)],
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

    let matches = &mut [false; 1];
    let slots = &mut [None; 1];
    let cache = Cache {
        jobs: vec![],
        visited: vec![0; MAX_SIZE_BYTES],
    };

    let input_at = InputAt {
        pos: 255,
        c: Char(0xFFFFFFFF),
        byte: Some(255),
        len: 1,
    };

    let mut bounded = Bounded {
        prog: &program,
        input: DummyInput {},
        matches,
        slots,
        m: &mut cache,
    };

    bounded.step(0, input_at);
}

struct DummyInput;

impl Input for DummyInput {
    fn at(&self, _i: usize) -> InputAt {
        InputAt {
            pos: 0,
            c: Char(0),
            byte: Some(0),
            len: 1,
        }
    }

    fn next_char(&self, _at: InputAt) -> Char {
        Char(0)
    }

    fn previous_char(&self, _at: InputAt) -> Char {
        Char(0)
    }

    fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
        false
    }

    fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
        None
    }

    fn len(&self) -> usize {
        1
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn as_bytes(&self) -> &[u8] {
        &[0]
    }
}

