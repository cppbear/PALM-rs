pub fn many_matches_at(
        &self,
        matches: &mut [bool],
        text: &[u8],
        start: usize,
    ) -> bool {
        use self::MatchType::*;
        if !self.is_anchor_end_match(text) {
            return false;
        }
        match self.ro.match_type {
            Literal(ty) => {
                debug_assert_eq!(matches.len(), 1);
                matches[0] = self.find_literals(ty, text, start).is_some();
                matches[0]
            }
            Dfa | DfaAnchoredReverse | DfaSuffix | DfaMany => {
                match dfa::Fsm::forward_many(
                    &self.ro.dfa,
                    self.cache,
                    matches,
                    text,
                    start,
                ) {
                    dfa::Result::Match(_) => true,
                    dfa::Result::NoMatch(_) => false,
                    dfa::Result::Quit => {
                        self.exec_nfa(
                            MatchNfaType::Auto,
                            matches,
                            &mut [],
                            false,
                            text,
                            start)
                    }
                }
            }
            Nfa(ty) => self.exec_nfa(ty, matches, &mut [], false, text, start),
            Nothing => false,
        }
    }