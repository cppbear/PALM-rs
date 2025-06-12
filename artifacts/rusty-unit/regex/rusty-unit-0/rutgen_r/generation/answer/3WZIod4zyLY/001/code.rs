// Answer 0

#[test]
fn test_lcp_non_empty() {
    struct TestFreqyPacked {
        lcp: String,
    }

    impl TestFreqyPacked {
        fn new(lcp: String) -> Self {
            TestFreqyPacked { lcp }
        }
    }

    let matcher = TestFreqyPacked::new("longest_common_prefix".to_string());
    let result = matcher.lcp();
    assert_eq!(result, "longest_common_prefix");
}

#[test]
fn test_lcp_empty() {
    struct TestFreqyPacked {
        lcp: String,
    }

    impl TestFreqyPacked {
        fn new(lcp: String) -> Self {
            TestFreqyPacked { lcp }
        }
    }

    let matcher = TestFreqyPacked::new("".to_string());
    let result = matcher.lcp();
    assert_eq!(result, "");
}

#[test]
#[should_panic]
fn test_lcp_invalid_reference() {
    struct TestFreqyPacked {
        lcp: String,
    }

    impl TestFreqyPacked {
        fn new(lcp: String) -> Self {
            TestFreqyPacked { lcp }
        }
    }

    let matcher: &TestFreqyPacked;
    {
        let temp_matcher = TestFreqyPacked::new("valid_prefix".to_string());
        matcher = &temp_matcher; // temp_matcher goes out of scope here
    }
    // This will cause a panic because matcher is a dangling reference
    let _ = matcher.lcp();
}

