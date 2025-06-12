fn backtrack(&mut self, start: InputAt) -> bool {
        // N.B. We use an explicit stack to avoid recursion.
        // To avoid excessive pushing and popping, most transitions are handled
        // in the `step` helper function, which only pushes to the stack when
        // there's a capture or a branch.
        let mut matched = false;
        self.m.jobs.push(Job::Inst { ip: 0, at: start });
        while let Some(job) = self.m.jobs.pop() {
            match job {
                Job::Inst { ip, at } => {
                    if self.step(ip, at) {
                        // Only quit if we're matching one regex.
                        // If we're matching a regex set, then mush on and
                        // try to find other matches (if we want them).
                        if self.prog.matches.len() == 1 {
                            return true;
                        }
                        matched = true;
                    }
                }
                Job::SaveRestore { slot, old_pos } => {
                    if slot < self.slots.len() {
                        self.slots[slot] = old_pos;
                    }
                }
            }
        }
        matched
    }