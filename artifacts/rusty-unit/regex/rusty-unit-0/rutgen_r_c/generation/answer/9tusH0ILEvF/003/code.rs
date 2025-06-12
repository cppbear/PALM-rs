// Answer 0

#[test]
fn test_check_match_with_valid_input() {
    struct TestBoyerMooreSearch {
        boyer_moore: BoyerMooreSearch,
    }

    impl TestBoyerMooreSearch {
        fn new(pattern: Vec<u8>) -> Self {
            TestBoyerMooreSearch {
                boyer_moore: BoyerMooreSearch::new(pattern),
            }
        }
    }

    let pattern = b"abc".to_vec();
    let haystack = b"xxabcxx";
    let searcher = TestBoyerMooreSearch::new(pattern.clone());

    assert!(searcher.boyer_moore.check_match(haystack, 5)); // 5 is the index of 'c'
}

#[test]
fn test_check_match_edge_case_no_match() {
    struct TestBoyerMooreSearch {
        boyer_moore: BoyerMooreSearch,
    }

    impl TestBoyerMooreSearch {
        fn new(pattern: Vec<u8>) -> Self {
            TestBoyerMooreSearch {
                boyer_moore: BoyerMooreSearch::new(pattern),
            }
        }
    }

    let pattern = b"abc".to_vec();
    let haystack = b"xxabcyy";
    let searcher = TestBoyerMooreSearch::new(pattern.clone());

    assert!(!searcher.boyer_moore.check_match(haystack, 6)); // 6 is the index of 'y'
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_check_match_with_panic_scenario() {
    struct TestBoyerMooreSearch {
        boyer_moore: BoyerMooreSearch,
    }

    impl TestBoyerMooreSearch {
        fn new(pattern: Vec<u8>) -> Self {
            TestBoyerMooreSearch {
                boyer_moore: BoyerMooreSearch::new(pattern),
            }
        }
    }

    let pattern = b"abc".to_vec();
    let haystack = b"xxabcxx";
    let searcher = TestBoyerMooreSearch::new(pattern.clone());

    // This should panic because the window_end is too small
    searcher.boyer_moore.check_match(haystack, 2); // 2 is an invalid index in this context
}

