// Answer 0

#[test]
fn test_find_freqy_packed_found() {
    struct FreqyPackedMatcher {
        data: Vec<u8>,
    }

    impl FreqyPackedMatcher {
        fn find(&self, haystack: &[u8]) -> Option<usize> {
            haystack.windows(self.data.len()).position(|window| window == self.data)
        }
    }

    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        FreqyPacked(FreqyPackedMatcher),
    }

    let pattern = FreqyPackedMatcher { data: b"test".to_vec() };
    let matcher = Matcher {
        matcher: MatcherType::FreqyPacked(pattern),
    };

    let haystack = b"This is a test string for testing.";
    let result = matcher.find(haystack);
    assert_eq!(result, Some((10, 14)));
}

#[test]
fn test_find_freqy_packed_not_found() {
    struct FreqyPackedMatcher {
        data: Vec<u8>,
    }

    impl FreqyPackedMatcher {
        fn find(&self, haystack: &[u8]) -> Option<usize> {
            haystack.windows(self.data.len()).position(|window| window == self.data)
        }
    }

    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        FreqyPacked(FreqyPackedMatcher),
    }

    let pattern = FreqyPackedMatcher { data: b"notfound".to_vec() };
    let matcher = Matcher {
        matcher: MatcherType::FreqyPacked(pattern),
    };

    let haystack = b"This is a test string for testing.";
    let result = matcher.find(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_freqy_packed_empty_haystack() {
    struct FreqyPackedMatcher {
        data: Vec<u8>,
    }

    impl FreqyPackedMatcher {
        fn find(&self, haystack: &[u8]) -> Option<usize> {
            haystack.windows(self.data.len()).position(|window| window == self.data)
        }
    }

    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        FreqyPacked(FreqyPackedMatcher),
    }

    let pattern = FreqyPackedMatcher { data: b"test".to_vec() };
    let matcher = Matcher {
        matcher: MatcherType::FreqyPacked(pattern),
    };

    let haystack: &[u8] = &[];
    let result = matcher.find(haystack);
    assert_eq!(result, None);
}

