fn exec_at_reverse(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        text: &[u8],
    ) -> Result<usize> {
        // The comments in `exec_at` above mostly apply here too. The main
        // difference is that we move backwards over the input and we look for
        // the longest possible match instead of the leftmost-first match.
        //
        // N.B. The code duplication here is regrettable. Efforts to improve
        // it without sacrificing performance are welcome. ---AG
        debug_assert!(self.prog.is_reverse);
        let mut result = Result::NoMatch(self.at);
        let (mut prev_si, mut next_si) = (self.start, self.start);
        let mut at = self.at;
        while at > 0 {
            while next_si <= STATE_MAX && at > 0 {
                // Argument for safety is in the definition of next_si.
                at -= 1;
                prev_si = unsafe { self.next_si(next_si, text, at) };
                if prev_si > STATE_MAX || at <= 4 {
                    mem::swap(&mut prev_si, &mut next_si);
                    break;
                }
                at -= 1;
                next_si = unsafe { self.next_si(prev_si, text, at) };
                if next_si > STATE_MAX {
                    break;
                }
                at -= 1;
                prev_si = unsafe { self.next_si(next_si, text, at) };
                if prev_si > STATE_MAX {
                    mem::swap(&mut prev_si, &mut next_si);
                    break;
                }
                at -= 1;
                next_si = unsafe { self.next_si(prev_si, text, at) };
            }
            if next_si & STATE_MATCH > 0 {
                next_si &= !STATE_MATCH;
                result = Result::Match(at + 1);
                if self.quit_after_match {
                    return result
                }
                self.last_match_si = next_si;
                prev_si = next_si;
                let cur = at;
                while (next_si & !STATE_MATCH) == prev_si && at >= 2 {
                    // Argument for safety is in the definition of next_si.
                    at -= 1;
                    next_si = unsafe {
                        self.next_si(next_si & !STATE_MATCH, text, at)
                    };
                }
                if at < cur {
                    result = Result::Match(at + 2);
                }
            } else if next_si >= STATE_UNKNOWN {
                if next_si == STATE_QUIT {
                    return Result::Quit;
                }
                let byte = Byte::byte(text[at]);
                prev_si &= STATE_MAX;
                self.at = at;
                next_si = match self.next_state(qcur, qnext, prev_si, byte) {
                    None => return Result::Quit,
                    Some(STATE_DEAD) => return result.set_non_match(at),
                    Some(si) => si,
                };
                debug_assert!(next_si != STATE_UNKNOWN);
                if next_si & STATE_MATCH > 0 {
                    next_si &= !STATE_MATCH;
                    result = Result::Match(at + 1);
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
        prev_si = match self.next_state(qcur, qnext, prev_si, Byte::eof()) {
            None => return Result::Quit,
            Some(STATE_DEAD) => return result.set_non_match(0),
            Some(si) => si,
        };
        debug_assert!(prev_si != STATE_UNKNOWN);
        if prev_si & STATE_MATCH > 0 {
            prev_si &= !STATE_MATCH;
            self.last_match_si = prev_si;
            result = Result::Match(0);
        }
        result
    }