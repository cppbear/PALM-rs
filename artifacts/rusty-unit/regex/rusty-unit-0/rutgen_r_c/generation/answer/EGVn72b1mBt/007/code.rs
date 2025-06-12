// Answer 0

#[test]
fn test_step_char_mismatch() {
    use std::sync::Arc;
    use std::collections::HashMap;

    let program = Program {
        insts: vec![
            prog::Inst::Char(InstChar {
                goto: 1,
                c: 'a',
            }),
            prog::Inst::Match(0),
        ],
        matches: vec![prog::InstPtr::from(0)],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: prog::InstPtr::from(0),
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

    let mut stack = vec![];

    struct TestInput {
        position: usize,
        character: Char,
        byte: Option<u8>,
        length: usize,
    }

    let at = InputAt {
        pos: 0,
        c: Char('b' as u32),
        byte: None,
        len: 1,
    };

    let at_next = InputAt {
        pos: 1,
        c: Char('c' as u32),
        byte: None,
        len: 1,
    };

    let mut nlist = Threads {
        set: sparse::SparseSet::new(),
        caps: vec![],
        slots_per_thread: 0,
    };
    
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default()];
    let mut thread_caps = vec![None];

    let mut fsm = Fsm {
        prog: &program,
        stack: &mut stack,
        input: TestInput {
            position: 0,
            character: Char(0),
            byte: None,
            length: 0,
        },
    };

    let result = fsm.step(&mut nlist, &mut matches, &mut slots, &mut thread_caps, 0, at, at_next);
    assert_eq!(result, false);
}

