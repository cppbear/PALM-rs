pub fn replacen<'t, R: Replacer>(
        &self,
        text: &'t str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'t, str> {
        // If we know that the replacement doesn't have any capture expansions,
        // then we can fast path. The fast path can make a tremendous
        // difference:
        //
        //   1) We use `find_iter` instead of `captures_iter`. Not asking for
        //      captures generally makes the regex engines faster.
        //   2) We don't need to look up all of the capture groups and do
        //      replacements inside the replacement string. We just push it
        //      at each match and be done with it.
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(text).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(text);
            }
            let mut new = String::with_capacity(text.len());
            let mut last_match = 0;
            for (i, m) in it {
                if limit > 0 && i >= limit {
                    break
                }
                new.push_str(&text[last_match..m.start()]);
                new.push_str(&rep);
                last_match = m.end();
            }
            new.push_str(&text[last_match..]);
            return Cow::Owned(new);
        }

        // The slower path, which we use if the replacement needs access to
        // capture groups.
        let mut it = self.captures_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(text);
        }
        let mut new = String::with_capacity(text.len());
        let mut last_match = 0;
        for (i, cap) in it {
            if limit > 0 && i >= limit {
                break
            }
            // unwrap on 0 is OK because captures only reports matches
            let m = cap.get(0).unwrap();
            new.push_str(&text[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
        }
        new.push_str(&text[last_match..]);
        Cow::Owned(new)
    }