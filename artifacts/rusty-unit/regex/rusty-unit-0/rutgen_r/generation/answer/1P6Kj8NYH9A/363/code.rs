// Answer 0

#[test]
#[should_panic]
fn test_exec_at_reverse_is_reverse_false() {
    struct TestDFA {
        prog: Program,
        at: usize,
        start: usize,
        quit_after_match: bool,
        last_match_si: usize,
    }

    struct Program {
        is_reverse: bool,
    }

    let mut dfa = TestDFA {
        prog: Program { is_reverse: false },
        at: 10,
        start: 0,
        quit_after_match: false,
        last_match_si: 0,
    };

    let mut qcur = SparseSet::new();
    let mut qnext = SparseSet::new();
    let text: &[u8] = b"test input";

    let _ = dfa.exec_at_reverse(&mut qcur, &mut qnext, text);
}

