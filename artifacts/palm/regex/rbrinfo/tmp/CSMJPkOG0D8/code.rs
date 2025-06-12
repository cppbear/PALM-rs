fn exec_at(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        text: &[u8],
    ) -> Result<usize> {
        // For the most part, the DFA is basically:
        //
        //   last_match = null
        //   while current_byte != EOF:
        //     si = current_state.next[current_byte]
        //     if si is match
        //       last_match = si
        //   return last_match
        //
        // However, we need to deal with a few things:
        //
        //   1. This is an *online* DFA, so the current state's next list
        //      may not point to anywhere yet, so we must go out and compute
        //      them. (They are then cached into the current state's next list
        //      to avoid re-computation.)
        //   2. If we come across a state that is known to be dead (i.e., never
        //      leads to a match), then we can quit early.
        //   3. If the caller just wants to know if a match occurs, then we
        //      can quit as soon as we know we have a match. (Full leftmost
        //      first semantics require continuing on.)
        //   4. If we're in the start state, then we can use a pre-computed set
        //      of prefix literals to skip quickly along the input.
        //   5. After the input is exhausted, we run the DFA on one symbol
        //      that stands for EOF. This is useful for handling empty width
        //      assertions.
        //   6. We can't actually do state.next[byte]. Instead, we have to do
        //      state.next[byte_classes[byte]], which permits us to keep the
        //      'next' list very small.
        //
        // Since there's a bunch of extra stuff we need to consider, we do some
        // pretty hairy tricks to get the inner loop to run as fast as
        // possible.
        debug_assert!(!self.prog.is_reverse);

        // The last match is the currently known ending match position. It is
        // reported as an index to the most recent byte that resulted in a
        // transition to a match state and is always stored in capture slot `1`
        // when searching forwards. Its maximum value is `text.len()`.
        let mut result = Result::NoMatch(self.at);
        let (mut prev_si, mut next_si) = (self.start, self.start);
        let mut at = self.at;
        while at < text.len() {
            // This is the real inner loop. We take advantage of special bits
            // set in the state pointer to determine whether a state is in the
            // "common" case or not. Specifically, the common case is a
            // non-match non-start non-dead state that has already been
            // computed. So long as we remain in the common case, this inner
            // loop will chew through the input.
            //
            // We also unroll the loop 4 times to amortize the cost of checking
            // whether we've consumed the entire input. We are also careful
            // to make sure that `prev_si` always represents the previous state
            // and `next_si` always represents the next state after the loop
            // exits, even if it isn't always true inside the loop.
            while next_si <= STATE_MAX && at < text.len() {
                // Argument for safety is in the definition of next_si.
                prev_si = unsafe { self.next_si(next_si, text, at) };
                at += 1;
                if prev_si > STATE_MAX || at + 2 >= text.len() {
                    mem::swap(&mut prev_si, &mut next_si);
                    break;
                }
                next_si = unsafe { self.next_si(prev_si, text, at) };
                at += 1;
                if next_si > STATE_MAX {
                    break;
                }
                prev_si = unsafe { self.next_si(next_si, text, at) };
                at += 1;
                if prev_si > STATE_MAX {
                    mem::swap(&mut prev_si, &mut next_si);
                    break;
                }
                next_si = unsafe { self.next_si(prev_si, text, at) };
                at += 1;
            }
            if next_si & STATE_MATCH > 0 {
                // A match state is outside of the common case because it needs
                // special case analysis. In particular, we need to record the
                // last position as having matched and possibly quit the DFA if
                // we don't need to keep matching.
                next_si &= !STATE_MATCH;
                result = Result::Match(at - 1);
                if self.quit_after_match {
                    return result;
                }
                self.last_match_si = next_si;
                prev_si = next_si;

                // This permits short-circuiting when matching a regex set.
                // In particular, if this DFA state contains only match states,
                // then it's impossible to extend the set of matches since
                // match states are final. Therefore, we can quit.
                if self.prog.matches.len() > 1 {
                    let state = self.state(next_si);
                    let just_matches = state.inst_ptrs()
                         .all(|ip| self.prog[ip].is_match());
                    if just_matches {
                        return result;
                    }
                }

                // Another inner loop! If the DFA stays in this particular
                // match state, then we can rip through all of the input
                // very quickly, and only recording the match location once
                // we've left this particular state.
                let cur = at;
                while (next_si & !STATE_MATCH) == prev_si
                    && at + 2 < text.len() {
                    // Argument for safety is in the definition of next_si.
                    next_si = unsafe {
                        self.next_si(next_si & !STATE_MATCH, text, at)
                    };
                    at += 1;
                }
                if at > cur {
                    result = Result::Match(at - 2);
                }
            } else if next_si & STATE_START > 0 {
                // A start state isn't in the common case because we may
                // what to do quick prefix scanning. If the program doesn't
                // have a detected prefix, then start states are actually
                // considered common and this case is never reached.
                debug_assert!(self.has_prefix());
                next_si &= !STATE_START;
                prev_si = next_si;
                at = match self.prefix_at(text, at) {
                    None => return Result::NoMatch(text.len()),
                    Some(i) => i,
                };
            } else if next_si >= STATE_UNKNOWN {
                if next_si == STATE_QUIT {
                    return Result::Quit;
                }
                // Finally, this corresponds to the case where the transition
                // entered a state that can never lead to a match or a state
                // that hasn't been computed yet. The latter being the "slow"
                // path.
                let byte = Byte::byte(text[at - 1]);
                // We no longer care about the special bits in the state
                // pointer.
                prev_si &= STATE_MAX;
                // Record where we are. This is used to track progress for
                // determining whether we should quit if we've flushed the
                // cache too much.
                self.at = at;
                next_si = match self.next_state(qcur, qnext, prev_si, byte) {
                    None => return Result::Quit,
                    Some(STATE_DEAD) => return result.set_non_match(at),
                    Some(si) => si,
                };
                debug_assert!(next_si != STATE_UNKNOWN);
                if next_si & STATE_MATCH > 0 {
                    next_si &= !STATE_MATCH;
                    result = Result::Match(at - 1);
                    if self.quit_after_match {
                        return result;
                    }
                    self.last_match_si = next_si;
                }
                prev_si = next_si;
            } else {
                prev_si = next_si;
            }
        }

        // Run the DFA once more on the special EOF senitnel value.
        // We don't care about the special bits in the state pointer any more,
        // so get rid of them.
        prev_si &= STATE_MAX;
        prev_si = match self.next_state(qcur, qnext, prev_si, Byte::eof()) {
            None => return Result::Quit,
            Some(STATE_DEAD) => return result.set_non_match(text.len()),
            Some(si) => si & !STATE_START,
        };
        debug_assert!(prev_si != STATE_UNKNOWN);
        if prev_si & STATE_MATCH > 0 {
            prev_si &= !STATE_MATCH;
            self.last_match_si = prev_si;
            result = Result::Match(text.len());
        }
        result
    }