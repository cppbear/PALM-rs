// Answer 0

#[test]
fn test_skip_loop_case1() {
    let pattern = vec![1];
    let haystack = vec![0; 20];
    let mut search = BoyerMooreSearch::new(pattern);
    search.skip_table.push(1); // Simulate non-zero skip value
    let result = search.skip_loop(&haystack, 0, 17);
}

#[test]
fn test_skip_loop_case2() {
    let pattern = vec![2];
    let haystack = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]; // Includes guard at the end
    let mut search = BoyerMooreSearch::new(pattern);
    search.skip_table.push(1);
    search.guard = 2; 
    search.guard_reverse_idx = 16; // Ensuring it returns correctly
    let result = search.skip_loop(&haystack, 0, 17);
}

#[test]
fn test_skip_loop_case3() {
    let pattern = vec![3];
    let haystack = vec![4; 32]; // Non-matching haystack
    haystack.push(3); // Adding guard
    let mut search = BoyerMooreSearch::new(pattern);
    search.skip_table.push(1);
    search.guard = 3;
    search.guard_reverse_idx = 16; 
    let result = search.skip_loop(&haystack, 0, 17);
}

#[test]
fn test_skip_loop_case4() {
    let pattern = vec![5];
    let haystack = vec![6; 30]; // Non-matching haystack
    haystack.push(5); // Adding guard
    let mut search = BoyerMooreSearch::new(pattern);
    search.skip_table.push(1);
    search.guard = 5;
    search.guard_reverse_idx = 16; 
    let result = search.skip_loop(&haystack, 0, 17);
}

#[test]
fn test_skip_loop_case5() {
    let pattern = vec![7];
    let mut haystack = vec![8; 32];
    haystack.push(7); // Add the guard at the back
    let mut search = BoyerMooreSearch::new(pattern);
    search.skip_table.push(1); // Force skip condition
    search.guard = 7;
    search.guard_reverse_idx = 16; 
    let result = search.skip_loop(&haystack, 0, 17);
}

