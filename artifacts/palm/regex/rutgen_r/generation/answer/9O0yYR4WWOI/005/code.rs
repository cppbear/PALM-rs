// Answer 0

#[test]
fn test_len_freqy_packed() {
    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        Empty,
        Bytes(Vec<u8>),
        FreqyPacked(u32),
        BoyerMoore(u32),
        AC(Box<str>),
        TeddySSSE3(Box<str>),
        TeddyAVX2(Box<str>),
    }

    let matcher = Matcher {
        matcher: MatcherType::FreqyPacked(42),
    };

    assert_eq!(matcher.len(), 1);
}

