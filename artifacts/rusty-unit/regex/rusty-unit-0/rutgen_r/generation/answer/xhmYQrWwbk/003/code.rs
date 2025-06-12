// Answer 0

#[test]
fn test_add_step_with_non_existing_ip_and_bytes_instruction() {
    struct TestStruct {
        prog: Vec<prog::Inst>,
        stack: Vec<FollowEpsilon>,
    }

    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 2];  // Assuming 2 slots for simplicity
    let mut ip: usize = 0;
    let at = InputAt::new(0);  // Assuming an appropriate initializer for InputAt

    let mut instance = TestStruct {
        prog: vec![prog::Inst::Bytes(vec![b'a']), prog::Inst::EmptyLook(prog::Inst::EmptyLookInst { goto: 1 }), prog::Inst::Match, prog::Inst::Save(prog::Inst::SaveInst { slot: 0, goto: 3 })],
        stack: Vec::new(),
    };

    // We need to ensure nlist.set does not contain ip
    assert!(!nlist.set.contains(&ip));

    // Run the function
    instance.add_step(&mut nlist, &mut thread_caps, ip, at);

    // Check that after running the function, the thread_caps is still None
    assert_eq!(thread_caps, vec![None, None]);
}

#[test]
#[should_panic]
fn test_add_step_with_existing_ip_should_panic() {
    struct TestStruct {
        prog: Vec<prog::Inst>,
        stack: Vec<FollowEpsilon>,
    }

    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 2];  
    let mut ip: usize = 0;
    let at = InputAt::new(0);

    let mut instance = TestStruct {
        prog: vec![prog::Inst::Bytes(vec![b'a']), prog::Inst::Match],
        stack: Vec::new(),
    };

    // Mark ip as visited
    nlist.set.insert(ip); 

    // Run the function, expecting it to panic due to existing ip
    instance.add_step(&mut nlist, &mut thread_caps, ip, at);
}

#[test]
fn test_add_step_with_slots_out_of_bounds() {
    struct TestStruct {
        prog: Vec<prog::Inst>,
        stack: Vec<FollowEpsilon>,
    }

    let mut nlist = Threads { set: std::collections::HashSet::new() };
    let mut thread_caps = vec![None; 1];
    let mut ip: usize = 0;
    let at = InputAt::new(0);

    let mut instance = TestStruct {
        prog: vec![prog::Inst::Save(prog::Inst::SaveInst { slot: 1, goto: 2 }), prog::Inst::Bytes(vec![b'a']), prog::Inst::Match],
        stack: Vec::new(),
    };

    assert!(!nlist.set.contains(&ip));

    // Run the function
    instance.add_step(&mut nlist, &mut thread_caps, ip, at);

    // Asserting that the thread_caps should not have been updated because slot was out of bounds
    assert_eq!(thread_caps, vec![None]);
}

