// Answer 0

#[test]
fn test_step_with_invalid_byte_match() {
    let prog = Program {
        insts: vec![
            Inst::Bytes(InstBytes { goto: 1, start: 0, end: 255 }),
            Inst::Match(0),
        ],
        matches: vec![false],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
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
    let mut cache = Cache { jobs: vec![], visited: vec![0; 8] };
    let input_at = InputAt { pos: 0, c: Char(97), byte: Some(1), len: 1 }; // Test with a byte that won't match.
    let mut bounded = Bounded {
        prog: &prog,
        input: DummyInput {},
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let ip = 0;
    let result = bounded.step(ip, input_at);
}

struct DummyInput;
impl Input for DummyInput {
    fn at(&self, _i: usize) -> InputAt { InputAt { pos: 0, c: Char(0), byte: Some(0), len: 1 } }
    fn next_char(&self, _at: InputAt) -> Char { Char(0) }
    fn previous_char(&self, _at: InputAt) -> Char { Char(0) }
    fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool { false }
    fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> { None }
    fn len(&self) -> usize { 1 }
    fn is_empty(&self) -> bool { false }
    fn as_bytes(&self) -> &[u8] { &[0] }
}

