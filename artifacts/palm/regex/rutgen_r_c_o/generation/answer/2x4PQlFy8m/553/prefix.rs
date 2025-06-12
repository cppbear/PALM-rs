// Answer 0

#[test]
fn test_exec_empty_clist_set() {
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
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::empty(),
        dfa_size_limit: 0,
    };

    let mut stack = vec![];
    let mut clist = Threads::new();
    clist.set.clear();
    
    let mut nlist = Threads::new();
    let mut matches = vec![false];
    let mut slots: Vec<Slot> = vec![];

    let at = InputAt {
        pos: 1,
        c: Char::None,
        byte: None,
        len: 1,
    };

    let fsm = Fsm { prog: &prog, stack: &mut stack, input: input };

    let result = fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, false, at);
}

