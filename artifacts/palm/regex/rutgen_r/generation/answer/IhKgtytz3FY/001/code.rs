// Answer 0

fn test_literal_searcher_new() {
    struct MockLiterals {
        complete: bool,
        longest_common_prefix: Vec<u8>,
        longest_common_suffix: Vec<u8>,
    }

    impl MockLiterals {
        fn all_complete(&self) -> bool {
            self.complete
        }

        fn longest_common_prefix(&self) -> &[u8] {
            &self.longest_common_prefix
        }

        fn longest_common_suffix(&self) -> &[u8] {
            &self.longest_common_suffix
        }
    }

    struct MockMatcher;

    // Test case where everything is complete and has defined prefixes/suffixes
    let literals = MockLiterals {
        complete: true,
        longest_common_prefix: vec![b'a', b'b'],
        longest_common_suffix: vec![b'y', b'z'],
    };
    
    let matcher = MockMatcher;

    let literal_searcher = new(literals, matcher);

    assert_eq!(literal_searcher.complete, true);
    assert_eq!(literal_searcher.lcp.data, vec![b'a', b'b']);
    assert_eq!(literal_searcher.lcs.data, vec![b'y', b'z']);
}

fn test_literal_searcher_new_incomplete() {
    struct MockLiterals {
        complete: bool,
        longest_common_prefix: Vec<u8>,
        longest_common_suffix: Vec<u8>,
    }

    impl MockLiterals {
        fn all_complete(&self) -> bool {
            self.complete
        }

        fn longest_common_prefix(&self) -> &[u8] {
            &self.longest_common_prefix
        }

        fn longest_common_suffix(&self) -> &[u8] {
            &self.longest_common_suffix
        }
    }

    struct MockMatcher;

    // Test case where literals are incomplete
    let literals = MockLiterals {
        complete: false,
        longest_common_prefix: vec![b'c', b'd'],
        longest_common_suffix: vec![b'x', b'y'],
    };

    let matcher = MockMatcher;

    let literal_searcher = new(literals, matcher);

    assert_eq!(literal_searcher.complete, false);
    assert_eq!(literal_searcher.lcp.data, vec![b'c', b'd']);
    assert_eq!(literal_searcher.lcs.data, vec![b'x', b'y']);
}

#[test]
fn test_all() {
    test_literal_searcher_new();
    test_literal_searcher_new_incomplete();
}

