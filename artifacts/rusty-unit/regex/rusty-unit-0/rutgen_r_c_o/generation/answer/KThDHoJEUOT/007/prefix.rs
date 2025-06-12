// Answer 0

#[test]
fn test_skip_loop_with_zero_skip() {
    let pattern = vec![b'a', b'b', b'c'];
    let search = BoyerMooreSearch::new(pattern);
    let haystack = vec![b'x', b'y', b'z', b'a', b'b', b'c'];
    let window_end = 0;
    let backstop = 10;

    search.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_with_edge_case_window_end() {
    let pattern = vec![b'a', b'b', b'c'];
    let search = BoyerMooreSearch::new(pattern);
    let haystack = vec![b'a', b'b', b'c', b'd'];
    let window_end = 0;
    let backstop = 1;

    search.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_with_large_window_end() {
    let pattern = vec![b'a', b'b', b'c'];
    let search = BoyerMooreSearch::new(pattern);
    let haystack = vec![b'a'; 255];
    let window_end = 200;
    let backstop = 255;

    search.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_with_repeated_characters() {
    let pattern = vec![b'x'];
    let search = BoyerMooreSearch::new(pattern);
    let haystack = vec![b'x', b'y', b'z', b'x', b'a'];
    let window_end = 3;
    let backstop = 5;

    search.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_with_minimal_length_haystack() {
    let pattern = vec![b'a'];
    let search = BoyerMooreSearch::new(pattern);
    let haystack = vec![b'c', b'a'];
    let window_end = 1;
    let backstop = 2;

    search.skip_loop(&haystack, window_end, backstop);
}

