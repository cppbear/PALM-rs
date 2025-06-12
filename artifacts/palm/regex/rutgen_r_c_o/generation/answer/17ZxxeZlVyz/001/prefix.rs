// Answer 0

#[test]
fn test_find_haystack_shorter_than_pattern_1() {
    let pattern = vec![1];
    let haystack = vec![];
    let bm_search = BoyerMooreSearch::new(pattern);
    bm_search.find(&haystack);
}

#[test]
fn test_find_haystack_shorter_than_pattern_2() {
    let pattern = vec![1, 2];
    let haystack = vec![0];
    let bm_search = BoyerMooreSearch::new(pattern);
    bm_search.find(&haystack);
}

#[test]
fn test_find_haystack_shorter_than_pattern_3() {
    let pattern = vec![1, 2, 3];
    let haystack = vec![0, 1, 2];
    let bm_search = BoyerMooreSearch::new(pattern);
    bm_search.find(&haystack);
}

#[test]
fn test_find_haystack_shorter_than_pattern_4() {
    let pattern = vec![3, 4, 5, 6];
    let haystack = vec![2, 3];
    let bm_search = BoyerMooreSearch::new(pattern);
    bm_search.find(&haystack);
}

#[test]
fn test_find_haystack_shorter_than_pattern_5() {
    let pattern = vec![5; 100];
    let haystack = vec![4; 99];
    let bm_search = BoyerMooreSearch::new(pattern);
    bm_search.find(&haystack);
}

