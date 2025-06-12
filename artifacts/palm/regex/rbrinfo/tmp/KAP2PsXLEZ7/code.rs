fn cached_state_key(
        &mut self,
        q: &SparseSet,
        state_flags: &mut StateFlags,
    ) -> Option<State> {
        use prog::Inst::*;

        // We need to build up enough information to recognize pre-built states
        // in the DFA. Generally speaking, this includes every instruction
        // except for those which are purely epsilon transitions, e.g., the
        // Save and Split instructions.
        //
        // Empty width assertions are also epsilon transitions, but since they
        // are conditional, we need to make them part of a state's key in the
        // cache.

        // Reserve 1 byte for flags.
        let mut insts = vec![0];
        let mut prev = 0;
        for &ip in q {
            let ip = usize_to_u32(ip);
            match self.prog[ip as usize] {
                Char(_) | Ranges(_) => unreachable!(),
                Save(_) | Split(_) => {}
                Bytes(_) => push_inst_ptr(&mut insts, &mut prev, ip),
                EmptyLook(_) => {
                    state_flags.set_empty();
                    push_inst_ptr(&mut insts, &mut prev, ip)
                }
                Match(_) => {
                    push_inst_ptr(&mut insts, &mut prev, ip);
                    if !self.continue_past_first_match() {
                        break;
                    }
                }
            }
        }
        // If we couldn't transition to any other instructions and we didn't
        // see a match when expanding NFA states previously, then this is a
        // dead state and no amount of additional input can transition out
        // of this state.
        if insts.len() == 1 && !state_flags.is_match() {
            None
        } else {
            let StateFlags(f) = *state_flags;
            insts[0] = f;
            Some(State { data: insts.into_boxed_slice() })
        }
    }