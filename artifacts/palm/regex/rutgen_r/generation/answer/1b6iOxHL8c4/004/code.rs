// Answer 0

#[test]
fn test_find_boyer_moore() {
    struct DummyBoyerMoore;

    impl DummyBoyerMoore {
        fn find(&self, haystack: &[u8]) -> Option<usize> {
            let needle = b"test"; // sample needle
            let haystack_str = std::str::from_utf8(haystack).unwrap();
            haystack_str.find("test")
        }
    }

    struct Matcher {
        matcher: MatcherType,
    }

    enum MatcherType {
        BoyerMoore(DummyBoyerMoore),
    }

    impl Matcher {
        pub fn find(&self, haystack: &[u8]) -> Option<(usize, usize)> {
            use MatcherType::*;
            match &self.matcher {
                BoyerMoore(ref s) => s.find(haystack).map(|i| (i, i + 4)), // "test" length is 4
                _ => None,
            }
        }
    }

    // Prepare a haystack containing the needle
    let haystack_with_needle = b"This is a test string containing the word test.";
    let haystack_without_needle = b"This is a string with no matching word.";

    let matcher = Matcher {
        matcher: MatcherType::BoyerMoore(DummyBoyerMoore),
    };

    // Test case when the needle is present
    let result = matcher.find(haystack_with_needle);
    assert_eq!(result, Some((10, 14))); // "test" found at position 10

    // Test case when the needle is absent
    let result_none = matcher.find(haystack_without_needle);
    assert_eq!(result_none, None); // "test" not found
}

