fn read_captures_at(
        &self,
        locs: &mut Locations,
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {
        let slots = as_slots(locs);
        for slot in slots.iter_mut() {
            *slot = None;
        }
        // If the caller unnecessarily uses this, then we try to save them
        // from themselves.
        match slots.len() {
            0 => return self.find_at(text, start),
            2 => {
                return self.find_at(text, start).map(|(s, e)| {
                    slots[0] = Some(s);
                    slots[1] = Some(e);
                    (s, e)
                });
            }
            _ => {} // fallthrough
        }
        if !self.is_anchor_end_match(text) {
            return None;
        }
        match self.ro.match_type {
            MatchType::Literal(ty) => {
                self.find_literals(ty, text, start).and_then(|(s, e)| {
                    self.captures_nfa_with_match(slots, text, s, e)
                })
            }
            MatchType::Dfa => {
                if self.ro.nfa.is_anchored_start {
                    self.captures_nfa(slots, text, start)
                } else {
                    match self.find_dfa_forward(text, start) {
                        dfa::Result::Match((s, e)) => {
                            self.captures_nfa_with_match(slots, text, s, e)
                        }
                        dfa::Result::NoMatch(_) => None,
                        dfa::Result::Quit => self.captures_nfa(slots, text, start),
                    }
                }
            }
            MatchType::DfaAnchoredReverse => {
                match self.find_dfa_anchored_reverse(text, start) {
                    dfa::Result::Match((s, e)) => {
                        self.captures_nfa_with_match(slots, text, s, e)
                    }
                    dfa::Result::NoMatch(_) => None,
                    dfa::Result::Quit => self.captures_nfa(slots, text, start),
                }
            }
            MatchType::DfaSuffix => {
                match self.find_dfa_reverse_suffix(text, start) {
                    dfa::Result::Match((s, e)) => {
                        self.captures_nfa_with_match(slots, text, s, e)
                    }
                    dfa::Result::NoMatch(_) => None,
                    dfa::Result::Quit => self.captures_nfa(slots, text, start),
                }
            }
            MatchType::Nfa(ty) => {
                self.captures_nfa_type(ty, slots, text, start)
            }
            MatchType::Nothing => None,
            MatchType::DfaMany => {
                unreachable!("BUG: RegexSet cannot be used with captures")
            }
        }
    }