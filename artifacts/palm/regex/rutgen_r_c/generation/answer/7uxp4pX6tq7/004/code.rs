// Answer 0

#[test]
fn test_fill_with_uncompiled_inst() {
    struct TestInstHole {
        slot: usize,
    }
    impl TestInstHole {
        fn new(slot: usize) -> Self {
            TestInstHole { slot }
        }
    }

    let inst_hole = InstHole::Save { slot: 1 };
    let mut maybe_inst = MaybeInst::Uncompiled(inst_hole);

    let goto: InstPtr = InstPtr(0); // Assuming InstPtr has a constructor that takes an usize

    maybe_inst.fill(goto);

    if let MaybeInst::Compiled(Inst::Save { slot, .. }) = maybe_inst {
        assert_eq!(slot, 1);
    } else {
        panic!("Expected MaybeInst to be Compiled with Save variant");
    }
}

#[test]
fn test_fill_with_split1() {
    let goto1: InstPtr = InstPtr(1);
    let goto: InstPtr = InstPtr(2);
    let mut maybe_inst = MaybeInst::Split1(goto1);

    maybe_inst.fill(goto);

    if let MaybeInst::Compiled(Inst::Split(inst_split)) = maybe_inst {
        assert_eq!(inst_split.goto1, goto1);
        assert_eq!(inst_split.goto2, goto);
    } else {
        panic!("Expected MaybeInst to be Compiled with Split variant");
    }
}

#[test]
fn test_fill_with_split2() {
    let goto2: InstPtr = InstPtr(3);
    let goto: InstPtr = InstPtr(4);
    let mut maybe_inst = MaybeInst::Split2(goto2);

    maybe_inst.fill(goto);

    if let MaybeInst::Compiled(Inst::Split(inst_split)) = maybe_inst {
        assert_eq!(inst_split.goto1, goto);
        assert_eq!(inst_split.goto2, goto2);
    } else {
        panic!("Expected MaybeInst to be Compiled with Split variant");
    }
}

#[should_panic]
fn test_fill_with_uncompiled_inst_panic() {
    let mut maybe_inst = MaybeInst::Compiled(Inst::Match(0)); // Start with a compiled instance
    
    let goto: InstPtr = InstPtr(5);
    
    maybe_inst.fill(goto); // This should panic
}

