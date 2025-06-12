// Answer 0

#[test]
fn test_fill_uncompiled() {
    struct InstPtr;

    struct UncompiledInst;

    impl UncompiledInst {
        fn fill(&self, _goto: InstPtr) -> Inst {
            Inst::CompiledInst
        }
    }

    enum MaybeInst {
        Uncompiled(UncompiledInst),
        Split1(InstPtr),
        Split2(InstPtr),
        Compiled(Inst),
    }

    enum Inst {
        CompiledInst,
        Split(InstSplit),
    }

    struct InstSplit {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    let mut maybe_inst = MaybeInst::Uncompiled(UncompiledInst);
    let goto = InstPtr;

    maybe_inst.fill(goto);

    if let MaybeInst::Compiled(Inst::CompiledInst) = maybe_inst {
        assert!(true);
    } else {
        assert!(false, "Expected MaybeInst::Compiled");
    }
}

#[test]
fn test_fill_split1() {
    struct InstPtr;

    enum MaybeInst {
        Uncompiled,
        Split1(InstPtr),
        Split2(InstPtr),
        Compiled(Inst),
    }

    enum Inst {
        Split(InstSplit),
    }

    struct InstSplit {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    impl MaybeInst {
        fn fill(&mut self, goto: InstPtr) {
            let filled = match *self {
                MaybeInst::Split1(goto1) => {
                    Inst::Split(InstSplit { goto1, goto2: goto })
                }
                _ => unreachable!(),
            };
            *self = MaybeInst::Compiled(filled);
        }
    }

    let goto1 = InstPtr;
    let goto2 = InstPtr;
    let mut maybe_inst = MaybeInst::Split1(goto1);

    maybe_inst.fill(goto2);

    if let MaybeInst::Compiled(Inst::Split(InstSplit { goto1, goto2 })) = maybe_inst {
        assert!(true);
    } else {
        assert!(false, "Expected MaybeInst::Compiled");
    }
}

#[test]
fn test_fill_split2() {
    struct InstPtr;

    enum MaybeInst {
        Uncompiled,
        Split1(InstPtr),
        Split2(InstPtr),
        Compiled(Inst),
    }

    enum Inst {
        Split(InstSplit),
    }

    struct InstSplit {
        goto1: InstPtr,
        goto2: InstPtr,
    }

    impl MaybeInst {
        fn fill(&mut self, goto: InstPtr) {
            let filled = match *self {
                MaybeInst::Split2(goto2) => {
                    Inst::Split(InstSplit { goto1: goto, goto2 })
                }
                _ => unreachable!(),
            };
            *self = MaybeInst::Compiled(filled);
        }
    }

    let goto1 = InstPtr;
    let goto2 = InstPtr;
    let mut maybe_inst = MaybeInst::Split2(goto2);

    maybe_inst.fill(goto1);

    if let MaybeInst::Compiled(Inst::Split(InstSplit { goto1, goto2 })) = maybe_inst {
        assert!(true);
    } else {
        assert!(false, "Expected MaybeInst::Compiled");
    }
}

