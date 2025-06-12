// Answer 0

#[test]
fn test_find_bytes_match() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Bytes(Box<SSet>),
        // other variants omitted for brevity
        Empty,
        FreqyPacked(Box<Vec<u8>>),
        BoyerMoore(Box<Vec<u8>>),
        AC(Box<Automaton>),
        TeddySSSE3(Box<Teddy>),
        TeddyAVX2(Box<Teddy>),
    }

    struct SSet {
        bytes: Vec<u8>,
    }

    impl SSet {
        fn find(&self, haystack: &[u8]) -> Option<usize> {
            haystack.windows(self.bytes.len()).position(|window| window == self.bytes.as_slice())
        }
    }

    let sset = SSet { bytes: vec![1, 2, 3] };
    let matcher = Matcher { matcher: MatcherType::Bytes(Box::new(sset)) };

    let haystack = vec![0, 1, 2, 3, 1, 2, 3, 4];
    assert_eq!(matcher.find(&haystack), Some((4, 7)));

    let haystack_no_match = vec![0, 4, 5, 6];
    assert_eq!(matcher.find(&haystack_no_match), None);

    let empty_haystack: &[u8] = &[];
    assert_eq!(matcher.find(empty_haystack), None);
}

