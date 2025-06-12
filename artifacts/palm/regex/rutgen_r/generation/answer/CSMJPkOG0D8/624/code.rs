// Answer 0

#[test]
fn test_exec_at_quit_case() {
    struct SparseSet;

    struct DFA {
        pub prog: Program,
        pub start: usize,
        pub at: usize,
        pub quit_after_match: bool,
        pub last_match_si: usize,
    }

    struct Program {
        pub is_reverse: bool,
        pub matches: Vec<MatchInfo>,
    }

    struct MatchInfo;

    impl DFA {
        fn next_state(&self, _qcur: &mut SparseSet, _qnext: &mut SparseSet, _prev_si: usize, _byte: Byte) -> Option<usize> {
            None
        }

        fn exec_at(
            &mut self,
            qcur: &mut SparseSet,
            qnext: &mut SparseSet,
            text: &[u8],
        ) -> Result {
            // Implementation from provided code...
            unimplemented!();
        }
    }

    enum Result {
        Quit,
        NoMatch(usize),
        Match(usize),
    }

    struct Byte;

    impl Byte {
        fn eof() -> Self {
            Byte
        }
        
        fn byte(_b: u8) -> Self {
            Byte
        }
    }

    let mut dfa = DFA {
        prog: Program {
            is_reverse: false,
            matches: Vec::new(),
        },
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: 0,
    };

    let mut qcur = SparseSet;
    let mut qnext = SparseSet;
    
    // Test input: text has a length of 0, simulating the case where at == text.len()
    let text = b""; // Empty slice as text which means at == 0

    let result = dfa.exec_at(&mut qcur, &mut qnext, text);
    
    assert_eq!(result, Result::Quit);
}

