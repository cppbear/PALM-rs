fn choose_match_type(&self, hint: Option<MatchType>) -> MatchType {
        use self::MatchType::*;
        if let Some(Nfa(_)) = hint {
            return hint.unwrap();
        }
        // If the NFA is empty, then we'll never match anything.
        if self.nfa.insts.is_empty() {
            return Nothing;
        }
        // If our set of prefixes is complete, then we can use it to find
        // a match in lieu of a regex engine. This doesn't quite work well in
        // the presence of multiple regexes, so only do it when there's one.
        //
        // TODO(burntsushi): Also, don't try to match literals if the regex is
        // partially anchored. We could technically do it, but we'd need to
        // create two sets of literals: all of them and then the subset that
        // aren't anchored. We would then only search for all of them when at
        // the beginning of the input and use the subset in all other cases.
        if self.res.len() == 1 {
            if self.nfa.prefixes.complete() {
                return if self.nfa.is_anchored_start {
                    Literal(MatchLiteralType::AnchoredStart)
                } else {
                    Literal(MatchLiteralType::Unanchored)
                };
            }
            if self.suffixes.complete() {
                return if self.nfa.is_anchored_end {
                    Literal(MatchLiteralType::AnchoredEnd)
                } else {
                    // This case shouldn't happen. When the regex isn't
                    // anchored, then complete prefixes should imply complete
                    // suffixes.
                    Literal(MatchLiteralType::Unanchored)
                };
            }
        }
        // If we can execute the DFA, then we totally should.
        if dfa::can_exec(&self.dfa) {
            // Regex sets require a slightly specialized path.
            if self.res.len() >= 2 {
                return DfaMany;
            }
            // If the regex is anchored at the end but not the start, then
            // just match in reverse from the end of the haystack.
            if !self.nfa.is_anchored_start && self.nfa.is_anchored_end {
                return DfaAnchoredReverse;
            }
            // If there's a longish suffix literal, then it might be faster
            // to look for that first.
            if self.should_suffix_scan() {
                return DfaSuffix;
            }
            // Fall back to your garden variety forward searching lazy DFA.
            return Dfa;
        }
        // We're so totally hosed.
        Nfa(MatchNfaType::Auto)
    }