fn find_literals(
        &self,
        ty: MatchLiteralType,
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {
        use self::MatchLiteralType::*;
        match ty {
            Unanchored => {
                let lits = &self.ro.nfa.prefixes;
                lits.find(&text[start..])
                    .map(|(s, e)| (start + s, start + e))
            }
            AnchoredStart => {
                let lits = &self.ro.nfa.prefixes;
                if !self.ro.nfa.is_anchored_start
                    || (self.ro.nfa.is_anchored_start && start == 0) {
                    lits.find_start(&text[start..])
                        .map(|(s, e)| (start + s, start + e))
                } else {
                    None
                }
            }
            AnchoredEnd => {
                let lits = &self.ro.suffixes;
                lits.find_end(&text[start..])
                    .map(|(s, e)| (start + s, start + e))
            }
        }
    }