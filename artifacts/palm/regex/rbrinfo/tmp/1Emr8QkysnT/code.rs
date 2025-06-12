pub fn approximate_size(&self) -> usize {
        use self::Matcher::*;
        match self.matcher {
            Empty => 0,
            Bytes(ref sset) => sset.approximate_size(),
            FreqyPacked(ref single) => single.approximate_size(),
            BoyerMoore(ref single) => single.approximate_size(),
            AC(ref aut) => aut.heap_bytes(),
            TeddySSSE3(ref ted) => ted.approximate_size(),
            TeddyAVX2(ref ted) => ted.approximate_size(),
        }
    }