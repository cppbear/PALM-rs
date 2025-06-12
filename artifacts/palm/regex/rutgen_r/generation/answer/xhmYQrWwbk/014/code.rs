// Answer 0

#[test]
fn test_add_step_with_empty_nlist() {
    struct MockProg {
        prog: Vec<prog::Inst>,
        input: InputAt,
        stack: Vec<FollowEpsilon>,
    }
    
    impl MockProg {
        fn new(prog: Vec<prog::Inst>, input: InputAt) -> Self {
            Self {
                prog,
                input,
                stack: Vec::new(),
            }
        }
    }
    
    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 2];
    let mut ip = 0;
    let at = InputAt::new(0); // Initialize with a position of 0
    
    let prog = vec![
        prog::Inst::EmptyLook(prog::EmptyLookInst { goto: 1 }),
        prog::Inst::Match(prog::MatchInst {}),
        prog::Inst::Save(prog::SaveInst { slot: 0, goto: 2 }),
        prog::Inst::Split(prog::SplitInst { goto1: 3, goto2: 4 }),
    ];
    
    let mut mock_prog = MockProg::new(prog, at);
    
    mock_prog.add_step(&mut nlist, &mut thread_caps, ip, at.clone());
    
    assert_eq!(nlist.set.len(), 1);
    assert!(thread_caps.iter().all(|slot| slot.is_none()));
    assert_eq!(mock_prog.stack.len(), 0);
}

#[test]
fn test_add_step_with_consecutive_calls() {
    struct MockProg {
        prog: Vec<prog::Inst>,
        input: InputAt,
        stack: Vec<FollowEpsilon>,
    }
    
    impl MockProg {
        fn new(prog: Vec<prog::Inst>, input: InputAt) -> Self {
            Self {
                prog,
                input,
                stack: Vec::new(),
            }
        }
        
        fn add_step(&mut self, nlist: &mut Threads, thread_caps: &mut [Option<usize>], ip: usize, at: InputAt) {
            // Implementation of the add_step function here
        }
    }

    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 2];
    let mut ip: usize = 0;
    let at = InputAt::new(0);
    
    let prog = vec![
        prog::Inst::EmptyLook(prog::EmptyLookInst { goto: 1 }),
        prog::Inst::Match(prog::MatchInst {}),
        prog::Inst::Save(prog::SaveInst { slot: 1, goto: 2 }),
        prog::Inst::Match(prog::MatchInst {}),
    ];
    
    let mut mock_prog = MockProg::new(prog, at);
    
    mock_prog.add_step(&mut nlist, &mut thread_caps, ip, at.clone());
    ip = 1; // Move to the Match instruction
    mock_prog.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert_eq!(nlist.set.len(), 2);
    assert_eq!(thread_caps[1], Some(at.pos()));
}

#[should_panic]
#[test]
fn test_add_step_with_out_of_bounds_ip() {
    struct MockProg {
        prog: Vec<prog::Inst>,
        input: InputAt,
        stack: Vec<FollowEpsilon>,
    }

    impl MockProg {
        fn new(prog: Vec<prog::Inst>, input: InputAt) -> Self {
            Self {
                prog,
                input,
                stack: Vec::new(),
            }
        }

        fn add_step(&mut self, nlist: &mut Threads, thread_caps: &mut [Option<usize>], ip: usize, at: InputAt) {
            // Implementation of the add_step function here
        }
    }
    
    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 2];
    let mut ip: usize = 10; // Intentionally setting out of bounds
    let at = InputAt::new(0);
    
    let prog = vec![
        prog::Inst::EmptyLook(prog::EmptyLookInst { goto: 1 }),
        prog::Inst::Match(prog::MatchInst {}),
    ];
    
    let mut mock_prog = MockProg::new(prog, at);
    
    mock_prog.add_step(&mut nlist, &mut thread_caps, ip, at);
}

