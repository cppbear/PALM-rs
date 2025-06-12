pub fn len(&self) -> usize {
        use self::Matcher::*;
        match self.matcher {
            Empty => 0,
            Bytes(ref sset) => sset.dense.len(),
            FreqyPacked(_) => 1,
            BoyerMoore(_) => 1,
            AC(ref aut) => aut.len(),
            TeddySSSE3(ref ted) => ted.len(),
            TeddyAVX2(ref ted) => ted.len(),
        }
    }