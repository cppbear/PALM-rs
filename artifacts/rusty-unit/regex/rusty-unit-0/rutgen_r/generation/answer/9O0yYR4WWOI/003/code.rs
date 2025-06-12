// Answer 0

#[test]
fn test_len_ac() {
    struct Auto {
        length: usize,
    }

    impl Auto {
        fn len(&self) -> usize {
            self.length
        }
    }

    enum Matcher {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(Vec<u8>),
        BoyerMoore(Vec<u8>),
        AC(Auto),
        TeddySSSE3(Vec<u8>),
        TeddyAVX2(Vec<u8>),
    }

    struct MatcherContainer {
        matcher: Matcher,
    }

    // Testing with the AC variant having a length of 5
    let auto = Auto { length: 5 };
    let matcher_container = MatcherContainer {
        matcher: Matcher::AC(auto),
    };

    assert_eq!(matcher_container.len(), 5);
}

#[test]
fn test_len_ac_zero() {
    struct Auto {
        length: usize,
    }

    impl Auto {
        fn len(&self) -> usize {
            self.length
        }
    }

    enum Matcher {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(Vec<u8>),
        BoyerMoore(Vec<u8>),
        AC(Auto),
        TeddySSSE3(Vec<u8>),
        TeddyAVX2(Vec<u8>),
    }

    struct MatcherContainer {
        matcher: Matcher,
    }

    // Testing with the AC variant having a length of 0
    let auto = Auto { length: 0 };
    let matcher_container = MatcherContainer {
        matcher: Matcher::AC(auto),
    };

    assert_eq!(matcher_container.len(), 0);
}

