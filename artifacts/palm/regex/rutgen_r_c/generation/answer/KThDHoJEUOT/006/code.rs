// Answer 0

#[test]
fn test_skip_loop_with_non_zero_skip() {
    let pattern = vec![b'a', b'b', b'c'];
    let skip_table = vec![0, 1, 2, 3];
    let guard = b'c';
    let guard_reverse_idx = 2;

    let search = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift: 1,
    };

    let haystack = b"xyzabcxyzabc";
    let window_end = 5; // Points to 'c'
    let backstop = 12; // Length of the haystack
    assert_eq!(search.skip_loop(haystack, window_end, backstop), Some(10)); // Next occurrence of 'c' at position 10
}

#[test]
fn test_skip_loop_with_zero_skip() {
    let pattern = vec![b'a', b'b', b'c'];
    let skip_table = vec![0, 0, 0, 0]; // All zero skip table
    let guard = b'c';
    let guard_reverse_idx = 2;

    let search = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift: 1,
    };

    let haystack = b"xyzxyzxyz";
    let window_end = 5; // No instances of 'c' to advance to
    let backstop = 12; // Length of the haystack
    assert_eq!(search.skip_loop(haystack, window_end, backstop), Some(5)); // Returns current window_end since skip is 0
}

#[test]
#[should_panic]
fn test_skip_loop_with_invalid_haystack_index() {
    let pattern = vec![b'a', b'b', b'c'];
    let skip_table = vec![0, 0, 0, 0]; // All zero skip table
    let guard = b'c';
    let guard_reverse_idx = 2;

    let search = BoyerMooreSearch {
        pattern,
        skip_table,
        guard,
        guard_reverse_idx,
        md2_shift: 1,
    };

    let haystack = b"xyzxyzxyz";
    let window_end = 100; // Invalid index, out of bounds
    let backstop = 12; // Length of the haystack
    search.skip_loop(haystack, window_end, backstop);
}

