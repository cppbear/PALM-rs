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

    let mut inst_hole = InstHole::Save { slot: 1 };
    let goto = InstPtr(42);
    let mut maybe_inst = MaybeInst::Uncompiled(inst_hole);

    maybe_inst.fill(goto);

    if let MaybeInst::Compiled(inst) = maybe_inst {
        match inst {
            Inst::Save(saved_inst) => {
                assert_eq!(saved_inst.slot, 1);
                assert_eq!(saved_inst.goto, goto);
            },
            _ => panic!("Expected Inst::Save"),
        }
    } else {
        panic!("Expected MaybeInst::Compiled");
    }
}

#[test]
fn test_fill_with_split1() {
    let goto1 = InstPtr(1);
    let goto2 = InstPtr(2);
    let mut maybe_inst = MaybeInst::Split1(goto1);
    maybe_inst.fill(goto2);

    if let MaybeInst::Compiled(inst) = maybe_inst {
        match inst {
            Inst::Split(inst_split) => {
                assert_eq!(inst_split.goto1, goto1);
                assert_eq!(inst_split.goto2, goto2);
            },
            _ => panic!("Expected Inst::Split"),
        }
    } else {
        panic!("Expected MaybeInst::Compiled");
    }
}

#[test]
fn test_fill_with_split2() {
    let goto1 = InstPtr(1);
    let goto2 = InstPtr(2);
    let mut maybe_inst = MaybeInst::Split2(goto2);
    maybe_inst.fill(goto1);

    if let MaybeInst::Compiled(inst) = maybe_inst {
        match inst {
            Inst::Split(inst_split) => {
                assert_eq!(inst_split.goto1, goto1);
                assert_eq!(inst_split.goto2, goto2);
            },
            _ => panic!("Expected Inst::Split"),
        }
    } else {
        panic!("Expected MaybeInst::Compiled");
    }
}

#[test]
#[should_panic(expected = "not all instructions were compiled!")]
fn test_fill_with_uncompiled_inst_unreachable() {
    let mut maybe_inst = MaybeInst::Compiled(Inst::Match(0));
    let goto = InstPtr(42);
    maybe_inst.fill(goto);
}

