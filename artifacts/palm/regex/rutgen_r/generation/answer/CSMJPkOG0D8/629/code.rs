// Answer 0

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
    }

    impl DFA {
        fn next_state(&self, _qcur: &mut SparseSet, _qnext: &mut SparseSet, _prev_si: usize, _byte: Byte) -> Option<usize> {
            None // Simulating the quit condition
        }
    }

    struct Byte;

    impl Byte {
        fn eof() -> Self {
            Byte // Simulating the EOF byte
        }
    }

    let mut qcur = SparseSet;
    let mut qnext = SparseSet;
    let text: &[u8] = b""; // Empty text to simulate at == text.len()

    let mut dfa = DFA {
        prog: Program { is_reverse: false },
        at: text.len(),
        start: 0,
        last_match_si: 0,
        quit_after_match: false,
    };

    let result = dfa.exec_at(&mut qcur, &mut qnext, text);
    assert!(matches!(result, Result::Quit));
}

