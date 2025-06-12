pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t [u8],
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, [u8]> {
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(text).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(text);
            }
            let mut new = Vec::with_capacity(text.len());
            let mut last_match = 0;
            for (i, m) in it {
                if limit > 0 && i >= limit {
                    break
                }
                new.extend_from_slice(&text[last_match..m.start()]);
                new.extend_from_slice(&rep);
                last_match = m.end();
            }
            new.extend_from_slice(&text[last_match..]);
            return Cow::Owned(new);
        }

        // The slower path, which we use if the replacement needs access to
        // capture groups.
        let mut it = self.captures_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(text);
        }
        let mut new = Vec::with_capacity(text.len());
        let mut last_match = 0;
        for (i, cap) in it {
            if limit > 0 && i >= limit {
                break
            }
            // unwrap on 0 is OK because captures only reports matches
            let m = cap.get(0).unwrap();
            new.extend_from_slice(&text[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
        }
        new.extend_from_slice(&text[last_match..]);
        Cow::Owned(new)
    }