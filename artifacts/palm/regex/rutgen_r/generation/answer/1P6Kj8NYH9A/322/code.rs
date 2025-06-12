// Answer 0

#[test]
fn test_exec_at_reverse_quit_condition() {
    struct TestDFA {
        prog: Program,
        at: usize,
        start: usize,
        last_match_si: usize,
        quit_after_match: bool,
    }

    impl TestDFA {
        fn new() -> Self {
            Self {
                prog: Program { is_reverse: true },
                at: 1,
                start: 0,
                last_match_si: 0,
                quit_after_match: false,
            }
        }

        fn next_state(&self, _qcur: &SparseSet, _qnext: &SparseSet, _prev_si: usize, _byte: Byte) -> Option<usize> {
            None
        }

        // Placeholder for `next_si` to simulate the condition
        unsafe fn next_si(&self, _state: usize, _text: &[u8], _at: usize) -> usize {
            STATE_MAX + 1 // Trigger the condition of next_si > STATE_MAX
        }
    }

    let mut dfa = TestDFA::new();
    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    let text: &[u8] = b"test";

    let result = dfa.exec_at_reverse(&mut qcur, &mut qnext, text);

    assert_eq!(result, Result::Quit);
}

#[test]
fn test_exec_at_reverse_match_condition() {
    struct TestDFA {
        prog: Program,
        at: usize,
        start: usize,
        last_match_si: usize,
        quit_after_match: bool,
    }

    impl TestDFA {
        fn new() -> Self {
            Self {
                prog: Program { is_reverse: true },
                at: 2,
                start: 0,
                last_match_si: 0,
                quit_after_match: false,
            }
        }

        fn next_state(&self, _qcur: &SparseSet, _qnext: &SparseSet, prev_si: usize, _byte: Byte) -> Option<usize> {
            if prev_si == 1 {
                Some(STATE_MATCH) // Simulate a match condition.
            } else {
                None
            }
        }

        unsafe fn next_si(&self, _state: usize, _text: &[u8], _at: usize) -> usize {
            1 // Returning a state that meets the condition
        }
    }

    let mut dfa = TestDFA::new();
    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    let text: &[u8] = b"test";

    let result = dfa.exec_at_reverse(&mut qcur, &mut qnext, text);

    assert!(matches!(result, Result::Match(_)));
}

