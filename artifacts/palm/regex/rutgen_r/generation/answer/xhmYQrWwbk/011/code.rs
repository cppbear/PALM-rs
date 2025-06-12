// Answer 0

#[test]
fn test_add_step_with_empty_list() {
    struct TestProg {
        prog: Vec<prog::Inst>,
        input: InputAt,
        stack: Vec<FollowEpsilon>,
    }

    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 3]; // Assuming we have 3 thread capture slots
    let mut ip = 0;
    let at = InputAt { /* initialize as required */ };
    
    let mut test_prog = TestProg {
        prog: vec![
            prog::Inst::Save(prog::SaveInst { slot: 0, goto: 1 }),
            prog::Inst::Match(prog::MatchInst {}),
        ],
        input: at,
        stack: vec![],
    };

    test_prog.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert!(nlist.set.contains(ip)); // Ensure the initial ip is contained
    assert_eq!(thread_caps[0], Some(at.pos())); // Ensure the slot has been updated
}

#[test]
fn test_add_step_with_multiple_saves() {
    struct TestProg {
        prog: Vec<prog::Inst>,
        input: InputAt,
        stack: Vec<FollowEpsilon>,
    }

    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 4]; // More slots for complexity
    let mut ip = 0;
    let at = InputAt { /* initialize as required */ };
    
    let mut test_prog = TestProg {
        prog: vec![
            prog::Inst::Save(prog::SaveInst { slot: 1, goto: 1 }),
            prog::Inst::Save(prog::SaveInst { slot: 2, goto: 2 }),
            prog::Inst::Match(prog::MatchInst {}),
        ],
        input: at,
        stack: vec![],
    };

    test_prog.add_step(&mut nlist, &mut thread_caps, ip, at);
    
    assert!(nlist.set.contains(ip)); // Ensure the initial ip is contained
    assert_eq!(thread_caps[1], Some(at.pos())); // Ensure slot 1 is updated
    assert_eq!(thread_caps[2], None); // Slot 2 should still be None
}

#[test]
#[should_panic]
fn test_add_step_with_invalid_slot() {
    struct TestProg {
        prog: Vec<prog::Inst>,
        input: InputAt,
        stack: Vec<FollowEpsilon>,
    }

    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 2]; // Only 2 slots
    let mut ip = 0;
    let at = InputAt { /* initialize as required */ };
    
    let mut test_prog = TestProg {
        prog: vec![
            prog::Inst::Save(prog::SaveInst { slot: 3, goto: 1 }), // Invalid slot
            prog::Inst::Match(prog::MatchInst {}),
        ],
        input: at,
        stack: vec![],
    };

    test_prog.add_step(&mut nlist, &mut thread_caps, ip, at);
} 

