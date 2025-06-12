// Answer 0

#[test]
fn test_skip_loop_case_1() {
    let pattern = vec![1, 2, 3, 4];
    let haystack = vec![5, 6, 1, 7, 8, 2, 9, 3, 10, 4];
    let backstop = 100;

    let search = BoyerMooreSearch::new(pattern);
    let result = search.skip_loop(&haystack, 17, backstop);
}

#[test]
fn test_skip_loop_case_2() {
    let pattern = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let haystack = vec![1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let backstop = 80;

    let search = BoyerMooreSearch::new(pattern);
    let result = search.skip_loop(&haystack, 18, backstop);
}

#[test]
fn test_skip_loop_case_3() {
    let pattern = vec![9, 8, 7];
    let haystack = vec![10, 11, 12, 9, 13, 8, 14, 7, 15, 16];
    let backstop = 64;

    let search = BoyerMooreSearch::new(pattern);
    let result = search.skip_loop(&haystack, 20, backstop);
}

#[test]
fn test_skip_loop_case_4() {
    let pattern = vec![2, 4, 6];
    let haystack = vec![100, 2, 200, 4, 300, 6, 400, 500, 600];
    let backstop = 32;

    let search = BoyerMooreSearch::new(pattern);
    let result = search.skip_loop(&haystack, 22, backstop);
}

#[test]
fn test_skip_loop_case_5() {
    let pattern = vec![255, 254, 253];
    let haystack = vec![0, 1, 255, 2, 254, 3, 253, 4, 5, 6];
    let backstop = 128;

    let search = BoyerMooreSearch::new(pattern);
    let result = search.skip_loop(&haystack, 25, backstop);
}

