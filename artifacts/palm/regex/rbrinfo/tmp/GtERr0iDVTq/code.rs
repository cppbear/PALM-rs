fn is_anchor_end_match(&self, text: &[u8]) -> bool {
        // Only do this check if the haystack is big (>1MB).
        if text.len() > (1<<20) && self.ro.nfa.is_anchored_end {
            let lcs = self.ro.suffixes.lcs();
            if lcs.len() >= 1 && !lcs.is_suffix(text) {
                return false;
            }
        }
        true
    }