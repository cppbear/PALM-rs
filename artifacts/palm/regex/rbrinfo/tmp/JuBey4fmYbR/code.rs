fn unwrap(self) -> Inst {
        match self {
            MaybeInst::Compiled(inst) => inst,
            _ => unreachable!("must be called on a compiled instruction, \
                               instead it was called on: {:?}", self),
        }
    }