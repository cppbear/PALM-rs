fn exec_(
        &mut self,
        mut clist: &mut Threads,
        mut nlist: &mut Threads,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        mut at: InputAt,
    ) -> bool {
        let mut matched = false;
        let mut all_matched = false;
        clist.set.clear();
        nlist.set.clear();
'LOOP:  loop {
            if clist.set.is_empty() {
                // Three ways to bail out when our current set of threads is
                // empty.
                //
                // 1. We have a match---so we're done exploring any possible
                //    alternatives. Time to quit. (We can't do this if we're
                //    looking for matches for multiple regexes, unless we know
                //    they all matched.)
                //
                // 2. If the expression starts with a '^' we can terminate as
                //    soon as the last thread dies.
                if (matched && matches.len() <= 1)
                    || all_matched
                    || (!at.is_start() && self.prog.is_anchored_start) {
                    break;
                }

                // 3. If there's a literal prefix for the program, try to
                //    jump ahead quickly. If it can't be found, then we can
                //    bail out early.
                if !self.prog.prefixes.is_empty() {
                    at = match self.input.prefix_at(&self.prog.prefixes, at) {
                        None => break,
                        Some(at) => at,
                    };
                }
            }

            // This simulates a preceding '.*?' for every regex by adding
            // a state starting at the current position in the input for the
            // beginning of the program only if we don't already have a match.
            if clist.set.is_empty()
                || (!self.prog.is_anchored_start && !all_matched) {
                self.add(&mut clist, slots, 0, at);
            }
            // The previous call to "add" actually inspects the position just
            // before the current character. For stepping through the machine,
            // we can to look at the current character, so we advance the
            // input.
            let at_next = self.input.at(at.next_pos());
            for i in 0..clist.set.len() {
                let ip = clist.set[i];
                if self.step(
                    &mut nlist,
                    matches,
                    slots,
                    clist.caps(ip),
                    ip,
                    at,
                    at_next,
                ) {
                    matched = true;
                    all_matched = all_matched || matches.iter().all(|&b| b);
                    if quit_after_match {
                        // If we only care if a match occurs (not its
                        // position), then we can quit right now.
                        break 'LOOP;
                    }
                    if self.prog.matches.len() == 1 {
                        // We don't need to check the rest of the threads
                        // in this set because we've matched something
                        // ("leftmost-first"). However, we still need to check
                        // threads in the next set to support things like
                        // greedy matching.
                        //
                        // This is only true on normal regexes. For regex sets,
                        // we need to mush on to observe other matches.
                        break;
                    }
                }
            }
            if at.is_end() {
                break;
            }
            at = at_next;
            mem::swap(clist, nlist);
            nlist.set.clear();
        }
        matched
    }