fn half_fill_split_goto1(&mut self, goto1: InstPtr) {
        let half_filled = match *self {
            MaybeInst::Split => goto1,
            _ => unreachable!("must be called on Split instruction, \
                               instead it was called on: {:?}", self),
        };
        *self = MaybeInst::Split1(half_filled);
    }