// Answer 0

#[test]
#[should_panic]
fn test_new_with_empty_pattern() {
    let pattern = Vec::new();
    let searcher = BoyerMooreSearch::new(pattern);
}

