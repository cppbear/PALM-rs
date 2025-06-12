// Answer 0

#[derive(Debug)]
struct MockSset {
    size: usize,
}

impl MockSset {
    pub fn approximate_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
enum Matcher {
    Empty,
    Bytes(MockSset),
    FreqyPacked(MockSset),
    BoyerMoore(MockSset),
    AC(MockSset),
    TeddySSSE3(MockSset),
    TeddyAVX2(MockSset),
}

struct Literal {
    matcher: Matcher,
}

impl Literal {
    pub fn approximate_size(&self) -> usize {
        use Matcher::*;
        match &self.matcher {
            Empty => 0,
            Bytes(ref sset) => sset.approximate_size(),
            FreqyPacked(ref single) => single.approximate_size(),
            BoyerMoore(ref single) => single.approximate_size(),
            AC(ref aut) => aut.approximate_size(),
            TeddySSSE3(ref ted) => ted.approximate_size(),
            TeddyAVX2(ref ted) => ted.approximate_size(),
        }
    }
}

#[test]
fn test_bytes_approximate_size() {
    let sset = MockSset { size: 1024 };
    let literal = Literal { matcher: Matcher::Bytes(sset) };
    assert_eq!(literal.approximate_size(), 1024);
}

#[test]
fn test_bytes_zero_size() {
    let sset = MockSset { size: 0 };
    let literal = Literal { matcher: Matcher::Bytes(sset) };
    assert_eq!(literal.approximate_size(), 0);
}

#[test]
fn test_bytes_large_size() {
    let sset = MockSset { size: usize::MAX };
    let literal = Literal { matcher: Matcher::Bytes(sset) };
    assert_eq!(literal.approximate_size(), usize::MAX);
}

