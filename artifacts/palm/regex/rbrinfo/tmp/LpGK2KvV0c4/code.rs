fn step(&mut self, mut ip: InstPtr, mut at: InputAt) -> bool {
        use prog::Inst::*;
        loop {
            // This loop is an optimization to avoid constantly pushing/popping
            // from the stack. Namely, if we're pushing a job only to run it
            // next, avoid the push and just mutate `ip` (and possibly `at`)
            // in place.
            if self.has_visited(ip, at) {
                return false;
            }
            match self.prog[ip] {
                Match(slot) => {
                    if slot < self.matches.len() {
                        self.matches[slot] = true;
                    }
                    return true;
                }
                Save(ref inst) => {
                    if let Some(&old_pos) = self.slots.get(inst.slot) {
                        // If this path doesn't work out, then we save the old
                        // capture index (if one exists) in an alternate
                        // job. If the next path fails, then the alternate
                        // job is popped and the old capture index is restored.
                        self.m.jobs.push(Job::SaveRestore {
                            slot: inst.slot,
                            old_pos: old_pos,
                        });
                        self.slots[inst.slot] = Some(at.pos());
                    }
                    ip = inst.goto;
                }
                Split(ref inst) => {
                    self.m.jobs.push(Job::Inst { ip: inst.goto2, at: at });
                    ip = inst.goto1;
                }
                EmptyLook(ref inst) => {
                    if self.input.is_empty_match(at, inst) {
                        ip = inst.goto;
                    } else {
                        return false;
                    }
                }
                Char(ref inst) => {
                    if inst.c == at.char() {
                        ip = inst.goto;
                        at = self.input.at(at.next_pos());
                    } else {
                        return false;
                    }
                }
                Ranges(ref inst) => {
                    if inst.matches(at.char()) {
                        ip = inst.goto;
                        at = self.input.at(at.next_pos());
                    } else {
                        return false;
                    }
                }
                Bytes(ref inst) => {
                    if let Some(b) = at.byte() {
                        if inst.matches(b) {
                            ip = inst.goto;
                            at = self.input.at(at.next_pos());
                            continue;
                        }
                    }
                    return false;
                }
            }
        }
    }