// Answer 0

#[test]
fn test_step_should_return_false_when_has_visited_is_false_and_char_does_not_match() {
    struct MockInput {
        characters: Vec<Char>,
        position: usize,
    }

    impl Input for MockInput {
        fn at(&self, i: usize) -> InputAt {
            InputAt {
                pos: i,
                c: self.characters[i],
                byte: None,
                len: 1,
            }
        }
        
        fn next_char(&self, at: InputAt) -> Char {
            self.at(at.next_pos()).char()
        }

        fn previous_char(&self, at: InputAt) -> Char {
            self.at(at.pos.checked_sub(1).unwrap_or(0)).char()
        }

        fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
            false
        }

        fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
            None
        }

        fn len(&self) -> usize {
            self.characters.len()
        }

        fn is_empty(&self) -> bool {
            self.characters.is_empty()
        }

        fn as_bytes(&self) -> &[u8] {
            unimplemented!()
        }
    }

    let prog = Program {
        insts: vec![
            Inst::Ranges(InstRanges {
                goto: 1,
                ranges: vec![('a', 'z')],
            }),
            Inst::Match(0),
        ],
        matches: vec![0],
        captures: vec![None],
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
        dfa_size_limit: 1024,
    };

    let mut matches = vec![false];
    let mut slots = vec![None; 1];
    let mut cache = Cache {
        jobs: vec![],
        visited: vec![0; 8],
    };

    let input = MockInput {
        characters: vec![Char(b'x' as u32)], // 'x' is not within 'a' to 'z'
        position: 0,
    };
    
    let mut bounded = Bounded {
        prog: &prog,
        input,
        matches: &mut matches,
        slots: &mut slots,
        m: &mut cache,
    };

    let result = bounded.step(0, InputAt { pos: 0, c: Char(b'x' as u32), byte: None, len: 1 });

    assert!(!result);
}

