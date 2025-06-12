// Answer 0

#[test]
fn test_step_with_empty_look() {
    struct DummyThreads;

    impl DummyThreads {
        fn new() -> Self {
            DummyThreads
        }
    }

    struct DummyInst {
        inst: prog::Inst,
    }

    struct DummyNFA {
        prog: Vec<DummyInst>,
    }

    let mut nlist = DummyThreads::new();
    
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 1];
    let mut thread_caps = vec![None; 1];

    let ip = 0;
    let at = InputAt::new('a'); // assuming InputAt::new(char) exists
    let at_next = InputAt::new('b');

    let nfa = DummyNFA {
        prog: vec![DummyInst {
            inst: prog::Inst::Save(0), // Save(_) is true
        }],
    };

    let result = nfa.step(
        &mut nlist,
        &mut matches,
        &mut slots,
        &mut thread_caps,
        ip,
        at,
        at_next,
    );

    assert!(!result);
}

#[test]
fn test_step_with_split() {
    struct DummyThreads;

    impl DummyThreads {
        fn new() -> Self {
            DummyThreads
        }
    }

    struct DummyInst {
        inst: prog::Inst,
    }

    struct DummyNFA {
        prog: Vec<DummyInst>,
    }

    let mut nlist = DummyThreads::new();
    
    let mut matches = vec![false; 1];
    let mut slots = vec![Slot::default(); 1];
    let mut thread_caps = vec![None; 1];

    let ip = 0;
    let at = InputAt::new('c'); // assuming InputAt::new(char) exists
    let at_next = InputAt::new('d');

    let nfa = DummyNFA {
        prog: vec![DummyInst {
            inst: prog::Inst::Split(0, 1), // Split(_) is true
        }],
    };

    let result = nfa.step(
        &mut nlist,
        &mut matches,
        &mut slots,
        &mut thread_caps,
        ip,
        at,
        at_next,
    );

    assert!(!result);
}

