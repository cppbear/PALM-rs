// Answer 0

#[test]
fn test_step_match_true() {
    use std::sync::Arc;
    use std::collections::HashMap;

    #[derive(Clone)]
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(self.data[i] as u32),
                byte: Some(self.data[i]),
                len: 1,
            }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            self.at(at.next_pos()).char()
        }

        fn previous_char(&self, at: InputAt) -> Char {
            self.at(at.pos.wrapping_sub(1)).char()
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            Some(at)
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

    let slot_count = 1;
    let matches = &mut [false; 1];
    let slots = &mut [None; 1];
    let prog = Program {
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        jobs: vec![],
        visited: vec![0; 8], // ensuring it has elements to possibly cache
    };

    let input = TestInput { data: vec![b'a'] };
    let mut bounded = Bounded {
        prog: &prog,
        input,
        matches,
        slots,
        m: &mut cache,
    };

    let ip = 0; // index of Match instruction
    let at = InputAt {
        pos: 0,
        c: Char(b'a' as u32),
        byte: Some(b'a'),
        len: 1,
    };

    let result = bounded.step(ip, at);
    assert!(result);
    assert!(matches[0]);
}

#[test]
fn test_step_slot_out_of_bounds() {
    use std::sync::Arc;
    use std::collections::HashMap;

    #[derive(Clone)]
    struct TestInput {
        data: Vec<u8>,
    }

    impl Input for TestInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: Char(self.data[i] as u32),
                byte: Some(self.data[i]),
                len: 1,
            }
        }

        fn next_char(&self, at: InputAt) -> Char {
            self.at(at.next_pos()).char()
        }

        fn previous_char(&self, at: InputAt) -> Char {
            self.at(at.pos.wrapping_sub(1)).char()
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            true
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt> {
            Some(at)
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

    let matches_len = 1;
    let mut matches = vec![false; matches_len];
    let slots = &mut [None; 1];
    
    let prog = Program {
        insts: vec![Inst::Match(matches_len)], // This causes out of bounds for matches
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache = Cache {
        jobs: vec![],
        visited: vec![0; 8], 
    };

    let input = TestInput { data: vec![b'a'] };
    let mut bounded = Bounded {
        prog: &prog,
        input,
        matches: &mut matches,
        slots,
        m: &mut cache,
    };

    let ip = 0; 
    let at = InputAt {
        pos: 0,
        c: Char(b'a' as u32),
        byte: Some(b'a'),
        len: 1,
    };

    let result = bounded.step(ip, at);
    assert!(result);
    assert!(!matches[0]); // Since the slot is out of bounds this should not be marked true
}

