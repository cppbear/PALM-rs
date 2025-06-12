// Answer 0

#[test]
fn test_len_empty() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Automaton),
        TeddySSSE3(Teddy),
        TeddyAVX2(Teddy),
    }

    struct Automaton {
        // Assuming this struct has a len method.
        len: usize,
    }

    struct Teddy {
        // Assuming this struct has a len method.
        len: usize,
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use MatcherType::*;
            match self.matcher {
                Empty => 0,
                Bytes(ref sset) => sset.len(),
                FreqyPacked(_) => 1,
                BoyerMoore(_) => 1,
                AC(ref aut) => aut.len,
                TeddySSSE3(ref ted) => ted.len,
                TeddyAVX2(ref ted) => ted.len,
            }
        }
    }

    let matcher = Matcher { matcher: MatcherType::Empty };
    assert_eq!(matcher.len(), 0);
}

#[test]
fn test_len_bytes() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Automaton),
        TeddySSSE3(Teddy),
        TeddyAVX2(Teddy),
    }

    struct Automaton {
        len: usize,
    }

    struct Teddy {
        len: usize,
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use MatcherType::*;
            match self.matcher {
                Empty => 0,
                Bytes(ref sset) => sset.len(),
                FreqyPacked(_) => 1,
                BoyerMoore(_) => 1,
                AC(ref aut) => aut.len,
                TeddySSSE3(ref ted) => ted.len,
                TeddyAVX2(ref ted) => ted.len,
            }
        }
    }

    let matcher = Matcher { matcher: MatcherType::Bytes(vec![1, 2, 3]) };
    assert_eq!(matcher.len(), 3);
}

#[test]
fn test_len_freqy_packed() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Automaton),
        TeddySSSE3(Teddy),
        TeddyAVX2(Teddy),
    }

    struct Automaton {
        len: usize,
    }

    struct Teddy {
        len: usize,
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use MatcherType::*;
            match self.matcher {
                Empty => 0,
                Bytes(ref sset) => sset.len(),
                FreqyPacked(_) => 1,
                BoyerMoore(_) => 1,
                AC(ref aut) => aut.len,
                TeddySSSE3(ref ted) => ted.len,
                TeddyAVX2(ref ted) => ted.len,
            }
        }
    }

    let matcher = Matcher { matcher: MatcherType::FreqyPacked(42) };
    assert_eq!(matcher.len(), 1);
}

#[test]
fn test_len_boyer_moore() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Automaton),
        TeddySSSE3(Teddy),
        TeddyAVX2(Teddy),
    }

    struct Automaton {
        len: usize,
    }

    struct Teddy {
        len: usize,
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use MatcherType::*;
            match self.matcher {
                Empty => 0,
                Bytes(ref sset) => sset.len(),
                FreqyPacked(_) => 1,
                BoyerMoore(_) => 1,
                AC(ref aut) => aut.len,
                TeddySSSE3(ref ted) => ted.len,
                TeddyAVX2(ref ted) => ted.len,
            }
        }
    }

    let matcher = Matcher { matcher: MatcherType::BoyerMoore(1) };
    assert_eq!(matcher.len(), 1);
}

#[test]
fn test_len_ac() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Automaton),
        TeddySSSE3(Teddy),
        TeddyAVX2(Teddy),
    }

    struct Automaton {
        len: usize,
    }

    struct Teddy {
        len: usize,
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use MatcherType::*;
            match self.matcher {
                Empty => 0,
                Bytes(ref sset) => sset.len(),
                FreqyPacked(_) => 1,
                BoyerMoore(_) => 1,
                AC(ref aut) => aut.len,
                TeddySSSE3(ref ted) => ted.len,
                TeddyAVX2(ref ted) => ted.len,
            }
        }
    }

    let aut = Automaton { len: 5 };
    let matcher = Matcher { matcher: MatcherType::AC(aut) };
    assert_eq!(matcher.len(), 5);
}

#[test]
fn test_len_teddy_ssse3() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Automaton),
        TeddySSSE3(Teddy),
        TeddyAVX2(Teddy),
    }

    struct Automaton {
        len: usize,
    }

    struct Teddy {
        len: usize,
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use MatcherType::*;
            match self.matcher {
                Empty => 0,
                Bytes(ref sset) => sset.len(),
                FreqyPacked(_) => 1,
                BoyerMoore(_) => 1,
                AC(ref aut) => aut.len,
                TeddySSSE3(ref ted) => ted.len,
                TeddyAVX2(ref ted) => ted.len,
            }
        }
    }

    let ted = Teddy { len: 10 };
    let matcher = Matcher { matcher: MatcherType::TeddySSSE3(ted) };
    assert_eq!(matcher.len(), 10);
}

#[test]
fn test_len_teddy_avx2() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Automaton),
        TeddySSSE3(Teddy),
        TeddyAVX2(Teddy),
    }

    struct Automaton {
        len: usize,
    }

    struct Teddy {
        len: usize,
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use MatcherType::*;
            match self.matcher {
                Empty => 0,
                Bytes(ref sset) => sset.len(),
                FreqyPacked(_) => 1,
                BoyerMoore(_) => 1,
                AC(ref aut) => aut.len,
                TeddySSSE3(ref ted) => ted.len,
                TeddyAVX2(ref ted) => ted.len,
            }
        }
    }

    let ted = Teddy { len: 15 };
    let matcher = Matcher { matcher: MatcherType::TeddyAVX2(ted) };
    assert_eq!(matcher.len(), 15);
}

