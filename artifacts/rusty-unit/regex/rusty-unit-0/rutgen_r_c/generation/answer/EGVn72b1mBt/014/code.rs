// Answer 0

#[test]
fn test_step_with_match_state() {
    use std::sync::Arc;
    use std::collections::HashMap;

    struct DummyInput {
        data: Vec<u8>,
    }

    impl Input for DummyInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(0),
                byte: self.data.get(i).copied(),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            Char(self.data[at.next_pos()] as u32)
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char(self.data[at.pos().checked_sub(1).unwrap_or(0)] as u32)
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &()) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &(), _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let program = Program {
        insts: vec![prog::Inst::Match(0)],
        matches: vec![InstPtr(0)],
        captures: vec![Some("capture".to_string())],
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
        prefixes: (),
        dfa_size_limit: 0,
    };

    let mut cache = ProgramCache::new();
    let mut matches = vec![false; 1];
    let mut slots = vec![0; 2];
    let mut thread_caps = vec![None; 2];
    let input = DummyInput { data: vec![b'a'] };
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input,
    };

    let input_at = input.at(0);
    let input_at_next = input.at(1);

    let result = fsm.step(
        &mut Threads { set: SparseSet::new(), caps: vec![], slots_per_thread: 2 },
        &mut matches,
        &mut slots,
        &mut thread_caps,
        0,
        input_at,
        input_at_next,
    );

    assert!(result);
    assert!(matches[0]);
}

#[test]
#[should_panic]
fn test_step_match_slot_out_of_bounds() {
    struct DummyInput {
        data: Vec<u8>,
    }

    impl Input for DummyInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(0),
                byte: self.data.get(i).copied(),
                len: self.data.len(),
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            Char(self.data[at.next_pos()] as u32)
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char(self.data[at.pos().checked_sub(1).unwrap_or(0)] as u32)
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &()) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &(), _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    let program = Program {
        insts: vec![prog::Inst::Match(1)],
        matches: vec![InstPtr(0)],
        captures: vec![Some("capture".to_string())],
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
        prefixes: (),
        dfa_size_limit: 0,
    };

    let input = DummyInput { data: vec![b'a'] };
    let mut fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input,
    };

    let input_at = input.at(0);
    let input_at_next = input.at(1);

    let mut matches = vec![false; 1];
    let mut slots = vec![0; 2];
    let mut thread_caps = vec![None; 2];

    fsm.step(
        &mut Threads { set: SparseSet::new(), caps: vec![], slots_per_thread: 2 },
        &mut matches,
        &mut slots,
        &mut thread_caps,
        0,
        input_at,
        input_at_next,
    );
}

