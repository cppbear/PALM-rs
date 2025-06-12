// Answer 0

#[test]
fn test_exec_at_reverse_quit() {
    struct DummyDFA {
        pub prog: Program,
        pub at: usize,
        pub start: usize,
        pub last_match_si: usize,
        pub quit_after_match: bool,
    }

    impl DummyDFA {
        pub fn next_si(&self, si: usize, text: &[u8], at: usize) -> usize {
            // Dummy implementation for testing
            if at < 5 {
                STATE_MAX + 1 // Forces next_si <= STATE_MAX to be false
            } else {
                si // Return the state directly otherwise
            }
        }

        pub fn next_state(
            &self,
            _qcur: &mut SparseSet,
            _qnext: &mut SparseSet,
            _prev_si: usize,
            _byte: Byte,
        ) -> Option<usize> {
            Some(STATE_DEAD) // Forces the condition to be tested
        }
    }

    struct Program {
        pub is_reverse: bool,
    }

    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    let text = b"sample text";
    
    let dfa = DummyDFA {
        prog: Program { is_reverse: true },
        at: 1, // at > 0
        start: 0,
        last_match_si: 0,
        quit_after_match: false,
    };

    let result = dfa.exec_at_reverse(&mut qcur, &mut qnext, text);
    assert_eq!(result, Result::Quit);
}

#[test]
fn test_exec_at_reverse_non_match() {
    struct DummyDFA {
        pub prog: Program,
        pub at: usize,
        pub start: usize,
        pub last_match_si: usize,
        pub quit_after_match: bool,
    }

    impl DummyDFA {
        pub fn next_si(&self, si: usize, text: &[u8], at: usize) -> usize {
            if at < 1 {
                STATE_MAX + 1 // Forces next_si <= STATE_MAX to be false
            } else {
                si // Return the state directly otherwise
            }
        }

        pub fn next_state(
            &self,
            _qcur: &mut SparseSet,
            _qnext: &mut SparseSet,
            _prev_si: usize,
            _byte: Byte,
        ) -> Option<usize> {
            None // Forces the condition to be tested
        }
    }

    struct Program {
        pub is_reverse: bool,
    }

    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    let text = b"sample text";
    
    let dfa = DummyDFA {
        prog: Program { is_reverse: true },
        at: 1,
        start: 0,
        last_match_si: 0,
        quit_after_match: false,
    };

    let result = dfa.exec_at_reverse(&mut qcur, &mut qnext, text);
    assert_eq!(result, Result::NoMatch(1));
}

