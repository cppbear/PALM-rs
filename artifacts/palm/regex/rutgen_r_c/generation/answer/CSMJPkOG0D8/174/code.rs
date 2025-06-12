// Answer 0

#[test]
fn test_exec_at_quit_result() {
    use std::collections::HashMap;

    struct MockCache {
        trans: MockTransitions,
    }

    struct MockTransitions {
        unknown: usize,
        dead: usize,
    }

    impl MockTransitions {
        fn next(&self, _si: StatePtr, _byte_class: usize) -> usize {
            self.unknown
        }
    }

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let mut cache = MockCache {
        trans: MockTransitions {
            unknown: STATE_UNKNOWN,
            dead: STATE_DEAD,
        },
    };

    let mut fsm = Fsm {
        prog: &program,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let text: &[u8] = &[0, 1]; // Length 2
    fsm.at = text.len(); // at < text.len() is false but we will manipulate inside our implementation.

    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
    match result {
        Result::Quit => (),
        _ => panic!("Expected Result::Quit"),
    }
}

