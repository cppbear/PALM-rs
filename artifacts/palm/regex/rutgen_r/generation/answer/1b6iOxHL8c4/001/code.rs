// Answer 0

#[test]
fn test_find_teddy_avx2_found() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        TeddyAVX2(Teddy),
    }

    struct Teddy {
        data: Vec<u8>,
    }

    impl Teddy {
        fn find(&self, haystack: &[u8]) -> Option<Match> {
            if let Some(pos) = haystack.windows(self.data.len()).position(|window| window == self.data.as_slice()) {
                return Some(Match { start: pos, end: pos + self.data.len() });
            }
            None
        }
    }

    struct Match {
        start: usize,
        end: usize,
    }

    let needle = b"abc";
    let haystack = b"xyzabcxyz";
    let teddy = Teddy { data: needle.to_vec() };
    let matcher = Matcher { matcher: MatcherType::TeddyAVX2(teddy) };

    let result = matcher.find(haystack);
    assert_eq!(result, Some((3, 6)));
}

#[test]
fn test_find_teddy_avx2_not_found() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        TeddyAVX2(Teddy),
    }

    struct Teddy {
        data: Vec<u8>,
    }

    impl Teddy {
        fn find(&self, haystack: &[u8]) -> Option<Match> {
            if let Some(pos) = haystack.windows(self.data.len()).position(|window| window == self.data.as_slice()) {
                return Some(Match { start: pos, end: pos + self.data.len() });
            }
            None
        }
    }

    struct Match {
        start: usize,
        end: usize,
    }

    let needle = b"abc";
    let haystack = b"xyzxyzxyz";
    let teddy = Teddy { data: needle.to_vec() };
    let matcher = Matcher { matcher: MatcherType::TeddyAVX2(teddy) };

    let result = matcher.find(haystack);
    assert_eq!(result, None);
} 

#[test]
fn test_find_teddy_avx2_empty_haystack() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        TeddyAVX2(Teddy),
    }

    struct Teddy {
        data: Vec<u8>,
    }

    impl Teddy {
        fn find(&self, haystack: &[u8]) -> Option<Match> {
            if let Some(pos) = haystack.windows(self.data.len()).position(|window| window == self.data.as_slice()) {
                return Some(Match { start: pos, end: pos + self.data.len() });
            }
            None
        }
    }

    struct Match {
        start: usize,
        end: usize,
    }

    let needle = b"abc";
    let haystack: &[u8] = b"";
    let teddy = Teddy { data: needle.to_vec() };
    let matcher = Matcher { matcher: MatcherType::TeddyAVX2(teddy) };

    let result = matcher.find(haystack);
    assert_eq!(result, None);
}

#[test]
fn test_find_teddy_avx2_match_at_start() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        TeddyAVX2(Teddy),
    }

    struct Teddy {
        data: Vec<u8>,
    }

    impl Teddy {
        fn find(&self, haystack: &[u8]) -> Option<Match> {
            if let Some(pos) = haystack.windows(self.data.len()).position(|window| window == self.data.as_slice()) {
                return Some(Match { start: pos, end: pos + self.data.len() });
            }
            None
        }
    }

    struct Match {
        start: usize,
        end: usize,
    }

    let needle = b"abc";
    let haystack = b"abcpqr";
    let teddy = Teddy { data: needle.to_vec() };
    let matcher = Matcher { matcher: MatcherType::TeddyAVX2(teddy) };

    let result = matcher.find(haystack);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_find_teddy_avx2_match_at_end() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        TeddyAVX2(Teddy),
    }

    struct Teddy {
        data: Vec<u8>,
    }

    impl Teddy {
        fn find(&self, haystack: &[u8]) -> Option<Match> {
            if let Some(pos) = haystack.windows(self.data.len()).position(|window| window == self.data.as_slice()) {
                return Some(Match { start: pos, end: pos + self.data.len() });
            }
            None
        }
    }

    struct Match {
        start: usize,
        end: usize,
    }

    let needle = b"xyz";
    let haystack = b"abcxyz";
    let teddy = Teddy { data: needle.to_vec() };
    let matcher = Matcher { matcher: MatcherType::TeddyAVX2(teddy) };

    let result = matcher.find(haystack);
    assert_eq!(result, Some((3, 6)));
}

#[test]
fn test_find_teddy_avx2_match_at_multiple_occurrences() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        TeddyAVX2(Teddy),
    }

    struct Teddy {
        data: Vec<u8>,
    }

    impl Teddy {
        fn find(&self, haystack: &[u8]) -> Option<Match> {
            if let Some(pos) = haystack.windows(self.data.len()).position(|window| window == self.data.as_slice()) {
                return Some(Match { start: pos, end: pos + self.data.len() });
            }
            None
        }
    }

    struct Match {
        start: usize,
        end: usize,
    }

    let needle = b"abc";
    let haystack = b"abcabcabc";
    let teddy = Teddy { data: needle.to_vec() };
    let matcher = Matcher { matcher: MatcherType::TeddyAVX2(teddy) };

    let result = matcher.find(haystack);
    assert_eq!(result, Some((0, 3)));
}

