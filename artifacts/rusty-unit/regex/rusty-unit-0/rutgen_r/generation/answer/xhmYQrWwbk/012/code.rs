// Answer 0

#[test]
fn test_add_step_with_empty_nlist() {
    struct MockProg {
        input: Vec<u8>,
        prog: Vec<prog::Inst>,
        stack: Vec<FollowEpsilon>,
    }

    struct Threads {
        set: std::collections::HashSet<usize>,
    }

    let mut nlist = Threads {
        set: std::collections::HashSet::new(),
    };
    
    let mut thread_caps = vec![None; 1]; // Thread capacity initialized with size 1
    let mut mock_prog = MockProg {
        input: vec![],
        prog: vec![prog::Inst::Save(prog::SaveInst { slot: 0, goto: 1 }), prog::Inst::Match(prog::MatchInst)],
        stack: vec![],
    };
    
    mock_prog.add_step(&mut nlist, &mut thread_caps, 0, InputAt { /* initialize InputAt as necessary */ });

    assert!(nlist.set.contains(0)); // Ensure the ip is in nlist after the call
    assert_eq!(thread_caps[0], None); // Check that thread_caps[0] was not mutated
}

#[test]
#[should_panic]
fn test_add_step_with_thread_caps_out_of_bounds() {
    struct MockProg {
        input: Vec<u8>,
        prog: Vec<prog::Inst>,
        stack: Vec<FollowEpsilon>,
    }

    struct Threads {
        set: std::collections::HashSet<usize>,
    }

    let mut nlist = Threads {
        set: std::collections::HashSet::new(),
    };

    let mut thread_caps = vec![None; 1]; // Thread capacity initialized with size 1
    let mut mock_prog = MockProg {
        input: vec![],
        prog: vec![prog::Inst::Save(prog::SaveInst { slot: 1, goto: 1 }), prog::Inst::Match(prog::MatchInst)],
        stack: vec![],
    };

    mock_prog.add_step(&mut nlist, &mut thread_caps, 0, InputAt { /* initialize InputAt as necessary */ });
}

#[test]
fn test_add_step_with_already_added_ip() {
    struct MockProg {
        input: Vec<u8>,
        prog: Vec<prog::Inst>,
        stack: Vec<FollowEpsilon>,
    }

    struct Threads {
        set: std::collections::HashSet<usize>,
    }

    let mut nlist = Threads {
        set: std::collections::HashSet::new(),
    };

    let mut thread_caps = vec![None; 1]; // Thread capacity initialized with size 1
    let mut mock_prog = MockProg {
        input: vec![],
        prog: vec![
            prog::Inst::Save(prog::SaveInst { slot: 0, goto: 1 }),
            prog::Inst::Save(prog::SaveInst { slot: 0, goto: 0 }), // Same `ip` as first
            prog::Inst::Match(prog::MatchInst)],
        stack: vec![],
    };

    mock_prog.add_step(&mut nlist, &mut thread_caps, 0, InputAt { /* initialize InputAt as necessary */ });
    
    assert!(nlist.set.contains(0)); // Ensure ip is in nlist
    assert_eq!(thread_caps[0], None); // Ensure no caps were set
}

