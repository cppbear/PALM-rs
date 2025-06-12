// Answer 0

#[test]
fn test_skip_loop_case_1() {
    let pattern = vec![1, 2, 3, 4];
    let searcher = BoyerMooreSearch::new(pattern);
    let haystack = vec![3; 64]; // fill with the same value to ensure loop progresses
    let window_end = 1;
    let backstop = 48;
    let _ = searcher.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_case_2() {
    let pattern = vec![5, 6, 7, 8];
    let searcher = BoyerMooreSearch::new(pattern);
    let haystack = (0..64).map(|x| x % 16).collect::<Vec<u8>>(); // values ensure skip != 0
    let window_end = 16;
    let backstop = 50;
    let _ = searcher.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_case_3() {
    let pattern = vec![9, 10, 11, 12];
    let searcher = BoyerMooreSearch::new(pattern);
    let haystack = (0..64).map(|x| (x + 1) % 16).collect::<Vec<u8>>(); // non-repeating to maximize skips
    let window_end = 32;
    let backstop = 64;
    let _ = searcher.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_case_4() {
    let pattern = vec![13, 14, 15, 16];
    let searcher = BoyerMooreSearch::new(pattern);
    let haystack = vec![0; 32].iter().map(|_| 1).collect::<Vec<u8>>(); // maximum repetition to test the panic condition
    let window_end = 1;
    let backstop = 32;
    let _ = searcher.skip_loop(&haystack, window_end, backstop);
}

#[test]
fn test_skip_loop_case_5() {
    let pattern = vec![17, 18, 19, 20];
    let searcher = BoyerMooreSearch::new(pattern);
    let haystack = (0..64).map(|x| (x * 2) % 16).collect::<Vec<u8>>(); // varied data to ensure multiple skips
    let window_end = 5;
    let backstop = 63;  // Ensure we reach the backstop condition
    let _ = searcher.skip_loop(&haystack, window_end, backstop);
}

