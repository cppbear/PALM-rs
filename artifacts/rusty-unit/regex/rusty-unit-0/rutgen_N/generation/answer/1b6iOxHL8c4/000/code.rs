// Answer 0

#[test]
fn test_find_empty() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        // Other variants are not needed for this test.
    }

    impl Matcher {
        pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            use MatcherType::*;
            match self.matcher {
                Empty => Some((0, 0)),
                // Other variants are not required for this test case.
            }
        }
    }

    let matcher = Matcher { matcher: MatcherType::Empty };
    let result = matcher.find(b"anyhaystack");
    assert_eq!(result, Some((0, 0)));
}

#[test]
fn test_find_bytes() {
    struct ByteSet;

    impl ByteSet {
        pub fn find(&self, haystack: &[u8]) -> Option<usize> {
            haystack.iter().position(|&x| x == b'a') // Example search for byte 'a'
        }
    }

    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Bytes(ByteSet),
        // Other variants are not needed for this test.
    }

    impl Matcher {
        pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            use MatcherType::*;
            match self.matcher {
                Bytes(ref sset) => sset.find(haystack).map(|i| (i, i + 1)),
                // Other variants are not required for this test case.
            }
        }
    }

    let matcher = Matcher { matcher: MatcherType::Bytes(ByteSet) };
    let result = matcher.find(b"abcdef");
    assert_eq!(result, Some((0, 1)));

    let result_not_found = matcher.find(b"bcdef");
    assert_eq!(result_not_found, None);
}

#[test]
fn test_find_freqy_packed() {
    struct FreqyPacked {
        pattern: &'static [u8],
    }

    impl FreqyPacked {
        pub fn find(&self, haystack: &[u8]) -> Option<usize> {
            haystack.windows(self.pattern.len()).position(|window| window == self.pattern)
        }

        pub fn len(&self) -> usize {
            self.pattern.len()
        }
    }

    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        FreqyPacked(FreqyPacked),
    }

    impl Matcher {
        pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            use MatcherType::*;
            match self.matcher {
                FreqyPacked(ref s) => s.find(haystack).map(|i| (i, i + s.len())),
            }
        }
    }

    let pattern = b"abc";
    let matcher = Matcher { matcher: MatcherType::FreqyPacked(FreqyPacked { pattern }) };
    let result = matcher.find(b"abcdef");
    assert_eq!(result, Some((0, 3)));

    let result_not_found = matcher.find(b"defgh");
    assert_eq!(result_not_found, None);
}

#[test]
fn test_find_boyer_moore() {
    struct BoyerMoore {
        pattern: &'static [u8],
    }

    impl BoyerMoore {
        pub fn find(&self, haystack: &[u8]) -> Option<usize> {
            haystack.windows(self.pattern.len()).position(|window| window == self.pattern)
        }

        pub fn len(&self) -> usize {
            self.pattern.len()
        }
    }

    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        BoyerMoore(BoyerMoore),
    }

    impl Matcher {
        pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            use MatcherType::*;
            match self.matcher {
                BoyerMoore(ref s) => s.find(haystack).map(|i| (i, i + s.len())),
            }
        }
    }

    let pattern = b"xyz";
    let matcher = Matcher { matcher: MatcherType::BoyerMoore(BoyerMoore { pattern }) };
    let result = matcher.find(b"abcdefxyz");
    assert_eq!(result, Some((6, 9)));

    let result_not_found = matcher.find(b"defgh");
    assert_eq!(result_not_found, None);
}

