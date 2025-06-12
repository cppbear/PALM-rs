// Answer 0

#[test]
fn test_find_empty_matcher() {
    struct EmptyMatcher;

    impl EmptyMatcher {
        fn find(&self, _: &[u8]) -> Option<(usize, usize)> {
            Some((0, 0))
        }
    }

    struct Matcher {
        matcher: EmptyMatcher,
    }

    let matcher = Matcher { matcher: EmptyMatcher };
    let result = matcher.find(b"any haystack");
    assert_eq!(result, Some((0, 0)));
}

