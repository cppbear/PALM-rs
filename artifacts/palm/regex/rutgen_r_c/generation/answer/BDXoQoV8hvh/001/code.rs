// Answer 0

#[test]
fn test_match_end() {
    struct TestMatch<'t> {
        inner: Match<'t>,
    }

    impl<'t> TestMatch<'t> {
        fn new(haystack: &'t [u8], start: usize, end: usize) -> Self {
            TestMatch {
                inner: Match::new(haystack, start, end),
            }
        }
        
        fn end(&self) -> usize {
            self.inner.end()
        }
    }

    // Test case 1: standard case
    let haystack1: &[u8] = b"hello world";
    let match1 = TestMatch::new(haystack1, 0, 5);
    assert_eq!(match1.end(), 5);

    // Test case 2: match at the end of the string
    let match2 = TestMatch::new(haystack1, 6, 11);
    assert_eq!(match2.end(), 11);

    // Test case 3: empty string match
    let haystack2: &[u8] = b"";
    let match3 = TestMatch::new(haystack2, 0, 0);
    assert_eq!(match3.end(), 0);

    // Test case 4: start and end the same
    let match4 = TestMatch::new(haystack1, 5, 5);
    assert_eq!(match4.end(), 5);

    // Test case 5: valid match with maximum end value
    let match5 = TestMatch::new(haystack1, 0, usize::MAX);
    assert_eq!(match5.end(), usize::MAX);
}

