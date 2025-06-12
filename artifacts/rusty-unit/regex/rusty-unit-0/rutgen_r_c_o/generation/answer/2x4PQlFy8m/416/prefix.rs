// Answer 0

#[test]
fn test_exec_with_empty_clist() {
    let program = Program {
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
    let quit_after_match = false;
    let at = InputAt {
        pos: 1,
        c: 'a',
        byte: None,
        len: 1,
    };

    let fsm = Fsm {
        prog: &program,
        stack: &mut vec![],
        input: CustomInput {},
    };

    fsm.exec_(&mut clist, &mut nlist, &mut matches, &mut slots, quit_after_match, at);
}

struct CustomInput;

impl Input for CustomInput {
    fn at(&self, i: usize) -> InputAt {
        InputAt {
            pos: i,
            c: 'a',
            byte: None,
            len: 1,
        }
    }
    
    fn next_char(&self, _at: InputAt) -> Char {
        'a'
    }

    fn previous_char(&self, _at: InputAt) -> Char {
        'a'
    }

    fn is_empty_match(&self, _at: InputAt, _empty: &InstEmptyLook) -> bool {
        false
    }
    
    fn prefix_at(&self, _prefixes: &LiteralSearcher, _at: InputAt) -> Option<InputAt> {
        Some(InputAt {
            pos: 1,
            c: 'a',
            byte: None,
            len: 1,
        })
    }

    fn len(&self) -> usize {
        10
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn as_bytes(&self) -> &[u8] {
        b"abcdefghij"
    }
}

