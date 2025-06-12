// Answer 0

#[test]
fn test_len_empty_matcher() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(Vec<u8>),
        BoyerMoore(Vec<u8>),
        AC(Box<dyn Automaton>),
        TeddySSSE3(Box<dyn Teddy>),
        TeddyAVX2(Box<dyn Teddy>),
    }

    impl Matcher {
        pub fn len(&self) -> usize {
            use self::MatcherType::*;
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

    // Testing for the Empty MatcherType
    let matcher = Matcher { matcher: MatcherType::Empty };
    assert_eq!(matcher.len(), 0);
}

