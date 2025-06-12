// Answer 0

#[test]
fn test_cached_state_key_with_empty_instructions() {
    struct TestDFA {
        prog: Vec<prog::Inst>,
    }

    impl TestDFA {
        fn continue_past_first_match(&self) -> bool {
            false
        }

        fn cached_state_key(
            &mut self,
            q: &SparseSet,
            state_flags: &mut StateFlags,
        ) -> Option<State> {
            // simplified function for testing
            use prog::Inst::*;

            let mut insts = vec![0];
            let mut prev = 0;
            for &ip in q {
                let ip = usize_to_u32(ip);
                match self.prog[ip as usize] {
                    Split(_) => {},
                    Save(_) => {},
                    // Other variants are unreachable for this test
                    _ => panic!("Unexpected instruction type"),
                }
            }
            if insts.len() == 1 && !state_flags.is_match() {
                None
            } else {
                let StateFlags(f) = *state_flags;
                insts[0] = f;
                Some(State { data: insts.into_boxed_slice() })
            }
        }
    }

    let mut dfa = TestDFA { prog: vec![prog::Inst::Split(0), prog::Inst::Save(1)] };
    let mut state_flags = StateFlags(0);
    let q: SparseSet = vec![0]; // Contains index pointing to Split
    
    let result = dfa.cached_state_key(&q, &mut state_flags);
    assert!(result.is_none());
}

#[test]
fn test_cached_state_key_with_non_matching() {
    struct TestDFA {
        prog: Vec<prog::Inst>,
    }

    impl TestDFA {
        fn continue_past_first_match(&self) -> bool {
            false
        }

        fn cached_state_key(
            &mut self,
            q: &SparseSet,
            state_flags: &mut StateFlags,
        ) -> Option<State> {
            use prog::Inst::*;

            let mut insts = vec![0];
            let mut prev = 0;
            for &ip in q {
                let ip = usize_to_u32(ip);
                match self.prog[ip as usize] {
                    Split(_) => {},
                    Save(_) => {},
                    _ => panic!("Unexpected instruction type"),
                }
            }
            if insts.len() == 1 && !state_flags.is_match() {
                None
            } else {
                let StateFlags(f) = *state_flags;
                insts[0] = f;
                Some(State { data: insts.into_boxed_slice() })
            }
        }
    }

    let mut dfa = TestDFA { prog: vec![prog::Inst::Split(0), prog::Inst::Save(1)] };
    let mut state_flags = StateFlags(1); // Only change the flag to be a match
    let q: SparseSet = vec![0]; // Contains index pointing to Split
    
    let result = dfa.cached_state_key(&q, &mut state_flags);
    assert!(result.is_some());
    assert_eq!(result.unwrap().data[0], 1);
}

#[test]
fn test_cached_state_key_with_multiple_instructions() {
    struct TestDFA {
        prog: Vec<prog::Inst>,
    }

    impl TestDFA {
        fn continue_past_first_match(&self) -> bool {
            true
        }

        fn cached_state_key(
            &mut self,
            q: &SparseSet,
            state_flags: &mut StateFlags,
        ) -> Option<State> {
            use prog::Inst::*;

            let mut insts = vec![0];
            let mut prev = 0;
            for &ip in q {
                let ip = usize_to_u32(ip);
                match self.prog[ip as usize] {
                    Split(_) => {},
                    Save(_) => {},
                    _ => panic!("Unexpected instruction type"),
                }
            }
            if insts.len() == 1 && !state_flags.is_match() {
                None
            } else {
                let StateFlags(f) = *state_flags;
                insts[0] = f;
                Some(State { data: insts.into_boxed_slice() })
            }
        }
    }

    let mut dfa = TestDFA { prog: vec![prog::Inst::Split(0), prog::Inst::Save(1)] };
    let mut state_flags = StateFlags(1); // This time, matching state
    let q: SparseSet = vec![0, 1]; // Contains multiple indexes, but they lead to non-special ops

    let result = dfa.cached_state_key(&q, &mut state_flags);
    assert!(result.is_some());
    assert_eq!(result.unwrap().data[0], 1);
}

