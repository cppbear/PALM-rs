// Answer 0

#[test]
fn test_iter_teddy_avx2() {
    struct TeddyAVX2 {
        patterns: Vec<String>,
    }

    impl TeddyAVX2 {
        fn patterns(&self) -> &[String] {
            &self.patterns
        }
    }

    enum Matcher {
        TeddyAVX2(TeddyAVX2),
        // Other matchers omitted for brevity
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(String),
        BoyerMoore(String),
        AC(Vec<String>),
        TeddySSSE3(TeddyAVX2),
    }

    struct MatcherContainer {
        matcher: Matcher,
    }

    struct LiteralIter<'a> {
        data: &'a [String],
    }

    impl MatcherContainer {
        pub fn iter(&self) -> LiteralIter {
            match &self.matcher {
                Matcher::TeddyAVX2(ref ted) => LiteralIter {
                    data: ted.patterns(),
                },
                _ => unreachable!(),
            }
        }
    }

    // Test setup
    let ted = TeddyAVX2 {
        patterns: vec!["pattern1".to_string(), "pattern2".to_string()],
    };

    let matcher_container = MatcherContainer {
        matcher: Matcher::TeddyAVX2(ted),
    };

    let iter = matcher_container.iter();

    assert_eq!(iter.data, &["pattern1".to_string(), "pattern2".to_string()]);
}

