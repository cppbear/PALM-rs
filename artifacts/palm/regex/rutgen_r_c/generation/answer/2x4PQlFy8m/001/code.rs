// Answer 0

#[test]
fn test_exec_empty_clist_with_matching() {
    struct MockInput {
        at_pos: usize,
        is_empty: bool,
    }

    impl Input for MockInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt { pos: i, c: Char {}, byte: None, len: 1 }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            Char {}
        }

        fn previous_char(&self, at: InputAt) -> Char {
            Char {}
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            Some(InputAt { pos: self.at_pos + 1, c: Char {}, byte: None, len: 1 })
        }

        fn len(&self) -> usize {
            1
        }

        fn is_empty(&self) -> bool {
            self.is_empty
        }

        fn as_bytes(&self) -> &[u8] {
            &[b'a']
        }
    }

    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let mut clist = Threads::new();
    let mut nlist = Threads::new();
    let mut matches = vec![false];
    let mut slots = vec![];

    let input = MockInput { at_pos: 0, is_empty: false };
    let mut fsm = Fsm {
        prog: &prog,
        stack: &mut vec![],
        input,
    };

    let result = fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, false, input.at(0));
    assert!(result);
}

