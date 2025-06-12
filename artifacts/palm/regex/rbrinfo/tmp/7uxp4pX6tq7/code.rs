fn fill(&mut self, goto: InstPtr) {
        let filled = match *self {
            MaybeInst::Uncompiled(ref inst) => inst.fill(goto),
            MaybeInst::Split1(goto1) => {
                Inst::Split(InstSplit { goto1: goto1, goto2: goto })
            }
            MaybeInst::Split2(goto2) => {
                Inst::Split(InstSplit { goto1: goto, goto2: goto2 })
            }
            _ => unreachable!("not all instructions were compiled! \
                               found uncompiled instruction: {:?}", self),
        };
        *self = MaybeInst::Compiled(filled);
    }