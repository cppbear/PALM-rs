// Answer 0

#[test]
fn test_check_match_with_valid_input() {
    struct Matcher {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    let matcher = Matcher {
        guard: b'A',
        guard_reverse_idx: 1,
        pattern: b"BCD".to_vec(),
    };

    let haystack = b"XYZABCDEF";
    let window_end = 8; // Position of 'A' in haystack

    assert!(matcher.check_match(haystack, window_end));
}

#[test]
fn test_check_match_with_edge_case() {
    struct Matcher {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    let matcher = Matcher {
        guard: b'A',
        guard_reverse_idx: 1,
        pattern: b"BCD".to_vec(),
    };

    let haystack = b"BCD";
    let window_end = 3; // Edge case where haystack length equals pattern length + guard index

    assert!(matcher.check_match(haystack, window_end));
}

#[test]
#[should_panic]
fn test_check_match_with_invalid_guard() {
    struct Matcher {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    let matcher = Matcher {
        guard: b'A',
        guard_reverse_idx: 2,
        pattern: b"BCD".to_vec(),
    };

    let haystack = b"XYZABCDEF";
    let window_end = 8; // Position of 'A' in haystack, but guard index will fail

    matcher.check_match(haystack, window_end);
}

#[test]
#[should_panic]
fn test_check_match_out_of_bounds() {
    struct Matcher {
        guard: u8,
        guard_reverse_idx: usize,
        pattern: Vec<u8>,
    }

    let matcher = Matcher {
        guard: b'A',
        guard_reverse_idx: 1,
        pattern: b"BCD".to_vec(),
    };

    let haystack = b"ABC";
    let window_end = 2; // Not enough elements to match the pattern

    matcher.check_match(haystack, window_end);
}

