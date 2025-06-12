// Answer 0

#[test]
fn test_iter_with_bytes() {
    struct BytesSet {
        dense: Vec<u8>,
    }
    
    enum Matcher {
        Empty,
        Bytes(BytesSet),
        FreqyPacked(Box<Pattern>),
        BoyerMoore(Box<Pattern>),
        AC(Box<AC>),
        TeddySSSE3(Box<Teddy>),
        TeddyAVX2(Box<Teddy>),
    }

    struct Pattern {
        pat: Vec<u8>,
    }

    struct AC {
        patterns: Vec<Vec<u8>>,
    }

    struct Teddy {
        patterns: Vec<Vec<u8>>,
    }

    struct MatcherWrapper {
        matcher: Matcher,
    }

    impl MatcherWrapper {
        pub fn iter(&self) -> LiteralIter {
            match &self.matcher {
                Matcher::Empty => LiteralIter::Empty,
                Matcher::Bytes(ref sset) => LiteralIter::Bytes(&sset.dense),
                Matcher::FreqyPacked(ref s) => LiteralIter::Single(&s.pat),
                Matcher::BoyerMoore(ref s) => LiteralIter::Single(&s.pattern),
                Matcher::AC(ref ac) => LiteralIter::AC(ac.patterns()),
                Matcher::TeddySSSE3(ref ted) => {
                    LiteralIter::TeddySSSE3(ted.patterns())
                }
                Matcher::TeddyAVX2(ref ted) => {
                    LiteralIter::TeddyAVX2(ted.patterns())
                }
            }
        }
    }

    enum LiteralIter<'a> {
        Empty,
        Bytes(&'a Vec<u8>),
        Single(&'a Vec<u8>),
        AC(Vec<Vec<u8>>),
        TeddySSSE3(Vec<Vec<u8>>),
        TeddyAVX2(Vec<Vec<u8>>),
    }

    let bytes_set = BytesSet {
        dense: vec![b'a', b'b', b'c', b'd'],
    };

    let matcher = MatcherWrapper {
        matcher: Matcher::Bytes(bytes_set),
    };

    if let LiteralIter::Bytes(dense) = matcher.iter() {
        assert_eq!(dense.as_slice(), &[b'a', b'b', b'c', b'd']);
    } else {
        panic!("Expected LiteralIter::Bytes, but got a different variant.");
    }
}

