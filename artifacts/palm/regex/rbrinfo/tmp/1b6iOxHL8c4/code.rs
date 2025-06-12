pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        use self::Matcher::*;
        match self.matcher {
            Empty => Some((0, 0)),
            Bytes(ref sset) => sset.find(haystack).map(|i| (i, i + 1)),
            FreqyPacked(ref s) => s.find(haystack).map(|i| (i, i + s.len())),
            BoyerMoore(ref s) => s.find(haystack).map(|i| (i, i + s.len())),
            AC(ref aut) => aut.find(haystack).next().map(|m| (m.start, m.end)),
            TeddySSSE3(ref t) => t.find(haystack).map(|m| (m.start, m.end)),
            TeddyAVX2(ref t) => t.find(haystack).map(|m| (m.start, m.end)),
        }
    }