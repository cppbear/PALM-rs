// Answer 0

#[derive(Debug)]
struct Matcher {
    matcher: MatcherType,
}

#[derive(Debug)]
enum MatcherType {
    Empty,
    Bytes(Vec<u8>),
    FreqyPacked(u8),
    BoyerMoore(u8),
    AC(Automaton),
    TeddySSSE3(Box<Teddy>),
    TeddyAVX2(Box<Teddy>),
}

#[derive(Debug)]
struct Automaton {
    len: usize,
}

impl Automaton {
    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug)]
struct Teddy {
    len: usize,
}

impl Teddy {
    fn len(&self) -> usize {
        self.len
    }
}

impl Matcher {
    pub fn len(&self) -> usize {
        use MatcherType::*;
        match self.matcher {
            Empty => 0,
            Bytes(ref sset) => sset.len(),
            FreqyPacked(_) => 1,
            BoyerMoore(_) => 1,
            AC(ref aut) => aut.len(),
            TeddySSSE3(ref ted) => ted.len(),
            TeddyAVX2(ref ted) => ted.len(),
        }
    }
}

#[test]
fn test_len_teddy_ssse3() {
    let teddy = Teddy { len: 5 };
    let matcher = Matcher {
        matcher: MatcherType::TeddySSSE3(Box::new(teddy)),
    };
    assert_eq!(matcher.len(), 5);
}

#[test]
fn test_len_empty() {
    let matcher = Matcher {
        matcher: MatcherType::Empty,
    };
    assert_eq!(matcher.len(), 0);
}

#[test]
fn test_len_bytes() {
    let matcher = Matcher {
        matcher: MatcherType::Bytes(vec![1, 2, 3]),
    };
    assert_eq!(matcher.len(), 3);
}

#[test]
fn test_len_freqy_packed() {
    let matcher = Matcher {
        matcher: MatcherType::FreqyPacked(1),
    };
    assert_eq!(matcher.len(), 1);
}

#[test]
fn test_len_boyer_moore() {
    let matcher = Matcher {
        matcher: MatcherType::BoyerMoore(1),
    };
    assert_eq!(matcher.len(), 1);
}

#[test]
fn test_len_automaton() {
    let automaton = Automaton { len: 4 };
    let matcher = Matcher {
        matcher: MatcherType::AC(automaton),
    };
    assert_eq!(matcher.len(), 4);
}

#[test]
fn test_len_teddy_avx2() {
    let teddy = Teddy { len: 3 };
    let matcher = Matcher {
        matcher: MatcherType::TeddyAVX2(Box::new(teddy)),
    };
    assert_eq!(matcher.len(), 3);
}

