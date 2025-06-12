// Answer 0

#[test]
fn test_select_guard_empty_pattern() {
    // This test will panic because the pattern is empty and accessing pattern[0] is invalid.
    let pattern: Vec<u8> = vec![];
    let guard = std::panic::catch_unwind(|| {
        select_guard(pattern.as_slice())
    });
    assert!(guard.is_err());
}

#[test]
fn test_select_guard_single_character() {
    let pattern: Vec<u8> = vec![b'a']; // Only one character, should be the rarest.
    let (rarest, rarest_rev_idx) = select_guard(pattern.as_slice());
    assert_eq!(rarest, b'a');
    assert_eq!(rarest_rev_idx, 0);
}

#[test]
fn test_select_guard_multiple_identical_characters() {
    let pattern: Vec<u8> = vec![b'a', b'a', b'a']; // All characters are the same.
    let (rarest, rarest_rev_idx) = select_guard(pattern.as_slice());
    assert_eq!(rarest, b'a');
    assert_eq!(rarest_rev_idx, 2); // The last occurrence index.
}

#[test]
fn test_select_guard_multiple_characters_no_repeats() {
    let pattern: Vec<u8> = vec![b'a', b'b', b'c']; // All characters are unique.
    let (rarest, rarest_rev_idx) = select_guard(pattern.as_slice());
    assert_eq!(rarest, b'a'); // Assuming 'a' is the rarest as per frequency assumptions.
    assert_eq!(rarest_rev_idx, 2); // Index for 'a' from the end is 2.
}

#[test]
fn test_select_guard_multiple_characters_with_frequencies() {
    let pattern: Vec<u8> = vec![b'a', b'b', b'a', b'c', b'b', b'a']; // Multiple frequencies.
    let (rarest, rarest_rev_idx) = select_guard(pattern.as_slice());
    assert_eq!(rarest, b'c'); // 'c' is the rarest.
    assert_eq!(rarest_rev_idx, 0); // Index for 'c' from the end is 0.
}

