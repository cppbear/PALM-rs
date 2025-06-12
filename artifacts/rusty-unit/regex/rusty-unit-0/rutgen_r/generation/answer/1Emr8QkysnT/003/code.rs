// Answer 0

#[derive(Debug)]
struct Aut { size: usize }

impl Aut {
    fn heap_bytes(&self) -> usize {
        self.size
    }
}

enum Matcher {
    Empty,
    Bytes(Vec<u8>),
    FreqyPacked(Box<dyn ApproximateSize>),
    BoyerMoore(Box<dyn ApproximateSize>),
    AC(Box<Aut>),
    TeddySSSE3(Box<dyn ApproximateSize>),
    TeddyAVX2(Box<dyn ApproximateSize>),
}

trait ApproximateSize {
    fn approximate_size(&self) -> usize;
}

struct MatcherWrapper {
    matcher: Matcher,
}

impl MatcherWrapper {
    pub fn approximate_size(&self) -> usize {
        use self::Matcher::*;
        match self.matcher {
            Empty => 0,
            Bytes(ref sset) => sset.len(),
            FreqyPacked(ref single) => single.approximate_size(),
            BoyerMoore(ref single) => single.approximate_size(),
            AC(ref aut) => aut.heap_bytes(),
            TeddySSSE3(ref ted) => ted.approximate_size(),
            TeddyAVX2(ref ted) => ted.approximate_size(),
        }
    }
}

#[test]
fn test_approximate_size_ac() {
    let aut = Box::new(Aut { size: 128 });
    let matcher = MatcherWrapper { matcher: Matcher::AC(aut) };
    assert_eq!(matcher.approximate_size(), 128);
}

#[test]
fn test_approximate_size_empty() {
    let matcher = MatcherWrapper { matcher: Matcher::Empty };
    assert_eq!(matcher.approximate_size(), 0);
}

#[test]
fn test_approximate_size_bytes() {
    let matcher = MatcherWrapper { matcher: Matcher::Bytes(vec![1, 2, 3]) };
    assert_eq!(matcher.approximate_size(), 3);
}

