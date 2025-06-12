fn exec_dfa_reverse_suffix(
        &self,
        text: &[u8],
        original_start: usize,
    ) -> Option<dfa::Result<(usize, usize)>> {
        use dfa::Result::*;

        let lcs = self.ro.suffixes.lcs();
        debug_assert!(lcs.len() >= 1);
        let mut start = original_start;
        let mut end = start;
        while end <= text.len() {
            start = end;
            end += match lcs.find(&text[end..]) {
                None => return Some(NoMatch(text.len())),
                Some(start) => start + lcs.len(),
            };
            match dfa::Fsm::reverse(
                &self.ro.dfa_reverse,
                self.cache,
                false,
                &text[start..end],
                end - start,
            ) {
                Match(0) | NoMatch(0) => return None,
                Match(s) => return Some(Match((s + start, end))),
                NoMatch(_) => continue,
                Quit => return Some(Quit),
            };
        }
        Some(NoMatch(text.len()))
    }