// Answer 0

#[test]
fn test_char_boundary_conditions() {
    let mut rng = Rng::with_seed(12345);

    // Test a range that satisfies all constraints
    let result = rng.char((Bound::Excluded('a'), Bound::Excluded('b')));
    assert_eq!(result, 'a');

    // Test a range where low == high
    let result = rng.char((Bound::Excluded('c'), Bound::Excluded('d')));
    assert_eq!(result, 'c');

    // Test a range where the gap is not included
    let result = rng.char((Bound::Excluded('A'), Bound::Excluded('Z')));
    assert!(result >= 'A' && result < 'Z');

    // Ensure it handles the suggestive conditions correctly
    let result = rng.char((Bound::Excluded('Z'), Bound::Excluded('[')));
    assert!(result >= 'Z' && result < '[');

    // Ensure that no values are returned in the surrogate range
    let result = rng.char((Bound::Excluded('!'), Bound::Excluded('~')));
    assert!(result >= '!' && result < '~');
}

#[test]
#[should_panic(expected = "empty range: excluded..excluded")]
fn test_char_empty_range() {
    let mut rng = Rng::with_seed(12345);
    rng.char((Bound::Excluded('a'), Bound::Excluded('a')));
}

#[test]
#[should_panic(expected = "empty range: excluded..excluded")]
fn test_char_high_low_reversed() {
    let mut rng = Rng::with_seed(12345);
    rng.char((Bound::Excluded('b'), Bound::Excluded('a')));
}

