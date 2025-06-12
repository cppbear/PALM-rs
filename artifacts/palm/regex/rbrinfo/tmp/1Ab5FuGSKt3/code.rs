fn exec_(&mut self, mut at: InputAt) -> bool {
        self.clear();
        // If this is an anchored regex at the beginning of the input, then
        // we're either already done or we only need to try backtracking once.
        if self.prog.is_anchored_start {
            return if !at.is_start() {
                false
            } else {
                self.backtrack(at)
            };
        }
        let mut matched = false;
        loop {
            if !self.prog.prefixes.is_empty() {
                at = match self.input.prefix_at(&self.prog.prefixes, at) {
                    None => break,
                    Some(at) => at,
                };
            }
            matched = self.backtrack(at) || matched;
            if matched && self.prog.matches.len() == 1 {
                return true;
            }
            if at.is_end() {
                break;
            }
            at = self.input.at(at.next_pos());
        }
        matched
    }