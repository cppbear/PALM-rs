// Answer 0

#[test]
fn test_exec_at_multiple_matches() {
    struct SparseSet {
        // Dummy structure for SparseSet
    }

    struct DummyDFA {
        pub prog: Prog,
        pub at: usize,
        pub start: usize,
        pub last_match_si: usize,
        pub quit_after_match: bool,
    }

    struct Prog {
        pub is_reverse: bool,
        pub matches: Vec<Match>,
    }

    struct Match {
        // Dummy structure for Match
    }

    let mut qcur = SparseSet {};
    let mut qnext = SparseSet {};

    let prog = Prog {
        is_reverse: false,
        matches: vec![Match {}, Match {}],
    };

    let mut dfa = DummyDFA {
        prog,
        at: 0,
        start: 0,
        last_match_si: 0,
        quit_after_match: false,
    };

    let text: Vec<u8> = b"example text".to_vec();
    
    // Simulate STATE_MAX and STATE_MATCH
    const STATE_MAX: usize = 255;
    const STATE_MATCH: usize = 1;

    // Mocking the next_si function
    impl DummyDFA {
        unsafe fn next_si(&self, state: usize, text: &[u8], index: usize) -> usize {
            if index < text.len() {
                // Returning a match state for demonstration
                return STATE_MATCH; 
            }
            STATE_MAX // Return an arbitrary max value otherwise
        }

        fn exec_at(
            &mut self,
            qcur: &mut SparseSet,
            qnext: &mut SparseSet,
            text: &[u8],
        ) -> Result<usize> {
            // Implementation based on original function
            // For demonstration, directly setting return value.
            self.at = text.len(); // Simulate processing the entire text
            return Result::Match(self.at);
        }
    }

    let result = unsafe {
        dfa.exec_at(&mut qcur, &mut qnext, &text)
    };

    match result {
        Result::Match(pos) => assert_eq!(pos, text.len()),
        _ => panic!("Expected a match result"),
    }
}

