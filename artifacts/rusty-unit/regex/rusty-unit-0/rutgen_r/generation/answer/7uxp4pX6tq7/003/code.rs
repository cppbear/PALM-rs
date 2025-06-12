// Answer 0

fn test_fill_split1() {
    struct InstSplit {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    enum Inst {
        Split(InstSplit),
    }

    enum MaybeInst {
        Uncompiled(Instruction),
        Split1(InstPtr),
        Split2(InstPtr),
        Compiled(Inst),
    }

    struct InstPtr(usize);

    struct Instruction;

    impl MaybeInst {
        fn fill(&mut self, goto: InstPtr) {
            let filled = match *self {
                MaybeInst::Uncompiled(ref inst) => inst.fill(goto),
                MaybeInst::Split1(goto1) => {
                    Inst::Split(InstSplit { goto1: goto1, goto2: goto })
                }
                MaybeInst::Split2(goto2) => {
                    Inst::Split(InstSplit { goto1: goto, goto2: goto2 })
                }
                _ => unreachable!("not all instructions were compiled! found uncompiled instruction: {:?}", self),
            };
            *self = MaybeInst::Compiled(filled);
        }
    }

    let goto1 = InstPtr(1);
    let goto2 = InstPtr(2);
    let mut maybe_inst = MaybeInst::Split1(goto1.0);

    maybe_inst.fill(goto2);

    match maybe_inst {
        MaybeInst::Compiled(Inst::Split(inst_split)) => {
            assert_eq!(inst_split.goto1.0, 1);
            assert_eq!(inst_split.goto2.0, 2);
        }
        _ => panic!("Expected Compiled instance but got {:?}", maybe_inst),
    }
}

fn test_fill_split2() {
    struct InstSplit {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    enum Inst {
        Split(InstSplit),
    }

    enum MaybeInst {
        Uncompiled(Instruction),
        Split1(InstPtr),
        Split2(InstPtr),
        Compiled(Inst),
    }

    struct InstPtr(usize);

    struct Instruction;

    impl MaybeInst {
        fn fill(&mut self, goto: InstPtr) {
            let filled = match *self {
                MaybeInst::Uncompiled(ref inst) => inst.fill(goto),
                MaybeInst::Split1(goto1) => {
                    Inst::Split(InstSplit { goto1: goto1, goto2: goto })
                }
                MaybeInst::Split2(goto2) => {
                    Inst::Split(InstSplit { goto1: goto, goto2: goto2 })
                }
                _ => unreachable!("not all instructions were compiled! found uncompiled instruction: {:?}", self),
            };
            *self = MaybeInst::Compiled(filled);
        }
    }

    let goto1 = InstPtr(3);
    let goto2 = InstPtr(4);
    let mut maybe_inst = MaybeInst::Split2(goto2.0);

    maybe_inst.fill(goto1);

    match maybe_inst {
        MaybeInst::Compiled(Inst::Split(inst_split)) => {
            assert_eq!(inst_split.goto1.0, 3);
            assert_eq!(inst_split.goto2.0, 4);
        }
        _ => panic!("Expected Compiled instance but got {:?}", maybe_inst),
    }
}

