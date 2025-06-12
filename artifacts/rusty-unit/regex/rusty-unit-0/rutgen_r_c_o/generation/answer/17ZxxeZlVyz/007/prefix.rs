// Answer 0

#[test]
fn test_find_pattern_matching_nonexistent() {
    let pattern = vec![1, 2, 3, 4, 5];
    let haystack = vec![6, 7, 8, 9, 10];

    let search = BoyerMooreSearch::new(pattern);
    let result = search.find(&haystack);
}

#[test]
fn test_find_exceeding_backstop() {
    let pattern = vec![a, b, c];
    let haystack = vec![e, f, g, h, i, j, k, l, m, n, o, p];

    let search = BoyerMooreSearch::new(pattern);
    let result = search.find(&haystack);
}

#[test]
fn test_find_window_end_skip_zero() {
    let pattern = vec![10];
    let haystack = vec![20, 30, 40, 50, 60, 70, 80, 90, 100];

    let search = BoyerMooreSearch::new(pattern);
    let result = search.find(&haystack);
}

#[test]
fn test_find_same_length_haystack_pattern() {
    let pattern = vec![5, 10, 15, 20, 25];
    let haystack = vec![30, 35, 40, 45, 50];

    let search = BoyerMooreSearch::new(pattern);
    let result = search.find(&haystack);
}

#[test]
fn test_find_haystack_distinct_bytes() {
    let pattern = vec![3, 6, 9];
    let haystack = vec![1, 2, 4, 5, 7, 8, 10];

    let search = BoyerMooreSearch::new(pattern);
    let result = search.find(&haystack);
}

#[test]
fn test_find_pattern_with_repeats_in_haystack() {
    let pattern = vec![10, 20];
    let haystack = vec![30, 30, 30, 30, 30];

    let search = BoyerMooreSearch::new(pattern);
    let result = search.find(&haystack);
}

#[test]
fn test_find_long_pattern() {
    let pattern = (1..=100).collect::<Vec<u8>>();
    let haystack = (101..=200).collect::<Vec<u8>>();

    let search = BoyerMooreSearch::new(pattern);
    let result = search.find(&haystack);
}

