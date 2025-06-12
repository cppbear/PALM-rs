fn fill_split(&mut self, goto1: InstPtr, goto2: InstPtr) {
        let filled = match *self {
            MaybeInst::Split => {
                Inst::Split(InstSplit { goto1: goto1, goto2: goto2 })
            }
            _ => unreachable!("must be called on Split instruction, \
                               instead it was called on: {:?}", self),
        };
        *self = MaybeInst::Compiled(filled);
    }