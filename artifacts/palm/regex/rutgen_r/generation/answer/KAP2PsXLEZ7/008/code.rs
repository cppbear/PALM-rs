// Answer 0

#[test]
fn test_cached_state_key_with_empty_look() {
    struct SparseSet(Vec<usize>);
    struct StateFlags(u8);
    struct State {
        data: Box<[u8]>,
    }
    
    struct DFA {
        prog: Vec<prog::Inst>,
    }
    
    impl DFA {
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
            for &ip in &q.0 {
                let ip = ip as usize;
                match self.prog[ip] {
                    EmptyLook(_) => {
                        state_flags.0 = 1; // representing a state that has an empty look
                        insts.push(ip as u8); // pushing the instruction pointer to insts
                    },
                    _ => {}
                }
            }
            if insts.len() == 1 && !state_flags.is_match() {
                None
            } else {
                insts[0] = state_flags.0;
                Some(State { data: insts.into_boxed_slice() })
            }
        }
    }
    
    impl StateFlags {
        fn is_match(&self) -> bool {
            self.0 > 0
        }
        
        fn set_empty(&mut self) {
            self.0 = 0;
        }
    }

    let mut dfa = DFA {
        prog: vec![
            prog::Inst::EmptyLook(0), // index 0 matches EmptyLook
            prog::Inst::EmptyLook(1), // index 1 also EmptyLook
        ],
    };

    let q = SparseSet(vec![0]); // empty look at index 0
    let mut state_flags = StateFlags(0);

    let result = dfa.cached_state_key(&q, &mut state_flags);
    
    assert!(result.is_some());
    let state = result.unwrap();
    assert_eq!(state.data.len(), 2); // should include the initial 0 and one for the empty look
}

#[test]
fn test_cached_state_key_no_instructions() {
    struct SparseSet(Vec<usize>);
    struct StateFlags(u8);
    struct State {
        data: Box<[u8]>,
    }
    
    struct DFA {
        prog: Vec<prog::Inst>,
    }
    
    impl DFA {
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
            for &ip in &q.0 {
                let ip = ip as usize;
                match self.prog[ip] {
                    EmptyLook(_) => {
                        state_flags.0 = 1; // representing a state that has an empty look
                        insts.push(ip as u8); // pushing the instruction pointer to insts
                    },
                    _ => {}
                }
            }
            if insts.len() == 1 && !state_flags.is_match() {
                None
            } else {
                insts[0] = state_flags.0;
                Some(State { data: insts.into_boxed_slice() })
            }
        }
    }
    
    impl StateFlags {
        fn is_match(&self) -> bool {
            self.0 > 0
        }
        
        fn set_empty(&mut self) {
            self.0 = 0;
        }
    }

    let mut dfa = DFA {
        prog: vec![], // no instructions
    };

    let q = SparseSet(vec![]); // empty set
    let mut state_flags = StateFlags(0);

    let result = dfa.cached_state_key(&q, &mut state_flags);
    
    assert!(result.is_none()); // should return None since insts.len() == 1 and !state_flags.is_match()
}

