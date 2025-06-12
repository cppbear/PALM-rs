// Answer 0

#[test]
fn test_exec_at_no_match() {
    struct SparseSet;

    struct DFA {
        prog: Program,
        at: usize,
        start: usize,
        last_match_si: usize,
        quit_after_match: bool,
    }

    struct Program {
        is_reverse: bool,
        matches: Vec<MatchState>,
    }

    struct MatchState;

    impl DFA {
        fn next_state(&self, _qcur: &mut SparseSet, _qnext: &mut SparseSet, prev_si: usize, _byte: Byte) -> Option<usize> {
            if prev_si == STATE_DEAD {
                None
            } else {
                Some(STATE_DEAD)
            }
        }

        fn prefix_at(&self, _text: &[u8], at: usize) -> Option<usize> {
            Some(at)
        }

        unsafe fn next_si(&self, si: usize, _text: &[u8], _at: usize) -> usize {
            si + 1
        }
    }

    #[derive(Debug)]
    struct Byte;

    impl Byte {
        fn eof() -> Self {
            Byte
        }
        
        fn byte(_value: u8) -> Self {
            Byte
        }
    }

    const STATE_UNKNOWN: usize = usize::MAX;
    const STATE_DEAD: usize = 0;
    const STATE_MAX: usize = 100;
    const STATE_MATCH: usize = 1;
    const STATE_START: usize = 2;

    let mut qcur = SparseSet;
    let mut qnext = SparseSet;
    let text: &[u8] = b"";
    
    let mut dfa = DFA {
        prog: Program {
            is_reverse: false,
            matches: vec![MatchState],
        },
        at: text.len(),
        start: 0,
        last_match_si: 0,
        quit_after_match: false,
    };

    let result = dfa.exec_at(&mut qcur, &mut qnext, text);
    match result {
        Ok(pos) => assert_eq!(pos, 0),
        Err(_) => panic!("Expected a match, but got an error"),
    }
}

#[test]
fn test_exec_at_quit_condition() {
    struct SparseSet;

    struct DFA {
        prog: Program,
        at: usize,
        start: usize,
        last_match_si: usize,
        quit_after_match: bool,
    }

    struct Program {
        is_reverse: bool,
        matches: Vec<MatchState>,
    }

    struct MatchState;

    impl DFA {
        fn next_state(&self, _qcur: &mut SparseSet, _qnext: &mut SparseSet, _prev_si: usize, _byte: Byte) -> Option<usize> {
            Some(STATE_DEAD) // Test case designed to hit a quit situation
        }

        fn prefix_at(&self, _text: &[u8], at: usize) -> Option<usize> {
            Some(at)
        }

        unsafe fn next_si(&self, si: usize, _text: &[u8], _at: usize) -> usize {
            si + 1
        }
    }

    #[derive(Debug)]
    struct Byte;

    impl Byte {
        fn eof() -> Self {
            Byte
        }
        
        fn byte(_value: u8) -> Self {
            Byte
        }
    }

    const STATE_UNKNOWN: usize = usize::MAX;
    const STATE_DEAD: usize = 0;
    const STATE_MAX: usize = 100;
    const STATE_MATCH: usize = 1;
    const STATE_START: usize = 2;

    let mut qcur = SparseSet;
    let mut qnext = SparseSet;
    let text: &[u8] = b"abc";

    let mut dfa = DFA {
        prog: Program {
            is_reverse: false,
            matches: vec![MatchState],
        },
        at: 0,
        start: 0,
        last_match_si: 0,
        quit_after_match: true,
    };

    let result = dfa.exec_at(&mut qcur, &mut qnext, text);
    match result {
        Ok(pos) => assert_eq!(pos, 0),
        Err(_) => panic!("Expected not to panic, but got an error"),
    }
}

